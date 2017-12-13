package chisel.axi

package object axi4lite {
  /** Tuple-type for bit ranges. */
  sealed case class BitRange(to: Int, from: Int) {
    require (to >= from && from >= 0, "BitRange: invalid range (%d, %d)".format(to, from))
    def overlapsWith(other: BitRange): Boolean = !(to < other.from || from > other.to)
  }
  /** Names for bit ranges. **/
  type BitfieldMap = Map[String, BitRange]
}
