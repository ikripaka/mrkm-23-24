val hexa = "5B88C41246790891C095E2878880342E88C79974303BD0400B090FE38A688356"
val hexb = "675215CC3E227D3216C056CFA8F8822BB486F788641E85E0DE77097E1DB049F1"
val hexm = "CEA42B987C44FA642D80AD9F51F10457690DEF10C83D0BC1BCEE12FC3B6093E3"
val a =  BigInt(hexa, 16)
val b = BigInt(hexb, 16)
val c = BigInt(hexm, 16)
//var b = BigDecimal("675215CC3E227D3216C056CFA8F8822BB486F788641E85E0DE77097E1DB049F1")
var as = (a + b).toString(16)
 def time[T](block: => T): T = {
   val i = 0
   val before = System.nanoTime()
   for (i <- 0 to 9998) {
     val result = block}
   val result = block
   val after = System.nanoTime()
   println("Elapsed time: " + ((after - before)/1000)/10000 + " mcs")
   result
 }
val mul = a * b
time((a+b).toString(16))
time((a-b).toString(16))
time((a*b).toString(16))
time((mul/b).toString(16))
time((a*a).toString(16))
time((a.modPow(c,b)).toString(16))
time((a.modInverse(c)).toString(16))