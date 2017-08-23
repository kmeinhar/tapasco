package chisel.axiutils
import  chisel3._
import  chisel3.util._
import  chisel3.iotesters.{ChiselFlatSpec, Driver, PeekPokeTester}
import  java.nio.file.Paths
import  chisel.axi._

class AxiFifoAdapterModule1(val fifoDepth: Int, val blockSize: Int)
                           (implicit axi: Axi4.Configuration) extends Module {
  val addrWidth = log2Ceil(axi.dataWidth * fifoDepth * blockSize / 8)
  val cfg = AxiSlaveModelConfiguration(size = Some(Math.pow(2, addrWidth:Int).toInt))
  val io = IO(new Bundle {
    val dqr = Output(Bool())
    val afa_deq_valid = Output(Bool())
    val afa_deq_bits  = Output(UInt(axi.dataWidth))
  })
  val afa  = Module (AxiFifoAdapter(fifoDepth = fifoDepth))
  val saxi = Module (new AxiSlaveModel(cfg))
  val dqr  = RegInit(true.B)

  afa.io.base      := 0.U
  //afa.io.deq.ready := true.B
  afa.io.deq.ready := dqr
  afa.io.maxi      <> saxi.io.saxi
  dqr              := io.dqr
  io.afa_deq_valid := afa.io.deq.valid
  io.afa_deq_bits  := afa.io.deq.bits
}

class AxiFifoAdapterModule1Test(m: AxiFifoAdapterModule1)
                               (implicit axi: Axi4.Configuration) extends PeekPokeTester(m) {
  import scala.util.Properties.{lineSeparator => NL}
  private var _cc = 0

  // re-define step to output progress info
  override def step(n: Int) {
    super.step(n)
    _cc += n
    if (cc % 1000 == 0) println("clock cycle: " + _cc)
  }

  // setup data
  println("prepping %d (%d x %d) mem elements ...".format(m.fifoDepth * m.blockSize, m.fifoDepth, m.blockSize))
  for (i <- 0 until m.fifoDepth * m.blockSize)
    pokeAt(m.saxi.mem, i % scala.math.pow(2, axi.dataWidth:Int).toInt, i)

  var res: List[BigInt] = List()
  var cc: Int = m.fifoDepth * m.blockSize * 10 // upper bound on cycles

  reset(10)
  poke(m.io.dqr, true)
  while (cc > 0 && res.length < m.fifoDepth * m.blockSize) {
    if (peek(m.io.afa_deq_valid) != 0) {
      val v: BigInt = peek(m.io.afa_deq_bits)
      res ++= List(v)
      poke(m.io.dqr, false)
      step(res.length % 20)
      poke(m.io.dqr, true)
    }
    step(1)
    cc -= 1
  }
  step(10) // settle

  res.zipWithIndex map (_ match { case (v, i) =>
      println("#%d: 0x%x (0b%s)".format(i, v, v.toString(2)))
    })
  
  for (i <- 0 until res.length if res(i) != peekAt(m.saxi.mem, i)) {
    val msg = "Mem[%03d] = %d (expected %d)".format(i, res(i), peekAt(m.saxi.mem, i))
    println(msg)
    expect(res(i) == peekAt(m.saxi.mem, i), msg)
  }
}

class AxiFifoAdapterSuite extends ChiselFlatSpec {
  def runTest(dataWidth: Int, fifoDepth: Int, blockSize: Int) {
    val dir = Paths.get("test").resolve("dw%d_fd%d_bs%d".format(dataWidth, fifoDepth, blockSize)).toString
    implicit val axi = Axi4.Configuration(dataWidth = DataWidth(dataWidth), addrWidth = AddrWidth(32))
    Driver.execute(Array("--fint-write-vcd", "--target-dir", dir),
                   () => new AxiFifoAdapterModule1(fifoDepth = fifoDepth, blockSize = blockSize))
      { m => new AxiFifoAdapterModule1Test(m) }
  }

  "checkDw32Fd1Bs256" should "be ok" in     { runTest(dataWidth = 32,  fifoDepth = 1,   blockSize = 256/1) }
  "checkDw32Fd8Bs32" should "be ok" in      { runTest(dataWidth = 32,  fifoDepth = 8,   blockSize = 256/8) }
  "checkDw8Fd8Bs32" should "be ok" in       { runTest(dataWidth = 8,   fifoDepth = 8,   blockSize = 256/8) }
  "checkDw8Fd2Bs128" should "be ok" in      { runTest(dataWidth = 8,   fifoDepth = 2,   blockSize = 256/2) }
  "checkDw64Fd16Bs512" should "be ok" in    { runTest(dataWidth = 64,  fifoDepth = 16,  blockSize = 512) }
  "checkDw128Fd128Bs1024" should "be ok" in { runTest(dataWidth = 128, fifoDepth = 128, blockSize = 1024/128) }
  // FIXME seems to work, but too slow
  // "checkDw8Fd1080Bs480" should "be ok" in   { runTest(dataWidth = 8,   fifoDepth = 256, blockSize = 480*4) }
}

