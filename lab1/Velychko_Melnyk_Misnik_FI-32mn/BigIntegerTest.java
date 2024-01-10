import java.math.BigInteger;
import java.util.Date;

public class Crypto {

    public static int numOfTimes = 10000;

    public static void main(String[] args) throws Exception {
        BigInteger A = new BigInteger("5B88C41246790891C095E2878880342E88C79974303BD0400B090FE38A688356", 16);
        BigInteger B = new BigInteger("675215CC3E227D3216C056CFA8F8822BB486F788641E85E0DE77097E1DB049F1", 16);
        BigInteger P = new BigInteger("CEA42B987C44FA642D80AD9F51F10457690DEF10C83D0BC1BCEE12FC3B6093E3", 16);
        BigInteger Sum = null;
        double sumTime = 0;
        for (int i = 0; i < numOfTimes; i++) {
            double startTime = System.currentTimeMillis();
            Sum = A.add(B);
            double elapsedTime = System.currentTimeMillis() - startTime;
            sumTime = elapsedTime + sumTime;
        }
        System.out.println("Execution time for sum " + (sumTime * 1000) / numOfTimes + "mcs");

        BigInteger Roz = null;
        sumTime = 0;
        for (int i = 0; i < numOfTimes; i++) {
            long startTime = System.currentTimeMillis();
            Roz = B.subtract(A);
            long elapsedTime = System.currentTimeMillis() - startTime;
            sumTime = elapsedTime + sumTime;
        }
        System.out.println("Execution time for sub " + (sumTime * 1000) / numOfTimes + "mcs");

        BigInteger Kvadr = null;
        sumTime = 0;
        for (int i = 0; i < numOfTimes; i++) {
            long startTime = System.currentTimeMillis();
            Kvadr = A.pow(2);
            long elapsedTime = System.currentTimeMillis() - startTime;
            sumTime = elapsedTime + sumTime;
        }
        System.out.println("Execution time for squa " + (sumTime * 1000) / numOfTimes + "mcs");

        BigInteger Mul = null;
        sumTime = 0;
        for (int i = 0; i < numOfTimes; i++) {
            long startTime = System.currentTimeMillis();
            Mul = A.multiply(B);
            long elapsedTime = System.currentTimeMillis() - startTime;
            sumTime = elapsedTime + sumTime;
        }
        System.out.println("Execution time for mul " + (sumTime * 1000) / numOfTimes + "mcs");

        BigInteger Div = null;
        BigInteger Amul = A.multiply(B);
        sumTime = 0;
        for (int i = 0; i < numOfTimes; i++) {
            long startTime = System.currentTimeMillis();
            Div = Amul.divide(B);
            long elapsedTime = System.currentTimeMillis() - startTime;
            sumTime = elapsedTime + sumTime;
        }
        System.out.println("Execution time for div " + (sumTime * 1000) / numOfTimes + "mcs");

        BigInteger PowMod = null;
        sumTime = 0;
        for (int i = 0; i < numOfTimes; i++) {
            long startTime = System.currentTimeMillis();
            PowMod = A.modPow(B, P);
            long elapsedTime = System.currentTimeMillis() - startTime;
            sumTime = elapsedTime + sumTime;
        }
        System.out.println("Execution time for pow mod " + (sumTime * 1000) / numOfTimes + "mcs");

        BigInteger ModInv = null;
        sumTime = 0;
        for (int i = 0; i < numOfTimes; i++) {
            long startTime = System.currentTimeMillis();
            ModInv = A.modInverse(P);
            long elapsedTime = System.currentTimeMillis() - startTime;
            sumTime = elapsedTime + sumTime;
        }
        System.out.println("Execution time for inverse " + (sumTime * 1000) / numOfTimes + "mcs");

        System.out.println("A+B");
        System.out.println(Sum.toString(16));
        System.out.println("B-A");
        System.out.println(Roz.toString(16));
        System.out.println("A^2");
        System.out.println(Kvadr.toString(16));
        System.out.println("A*B");
        System.out.println(Mul.toString(16));
        System.out.println("A/B");
        System.out.println(Div.toString(16));
        System.out.println("A^Bmod P");
        System.out.println(PowMod.toString(16));
        System.out.println("A^-1modP");
        System.out.println(ModInv.toString(16));

        System.out.println(A.mod(P).toString(16));


        System.out.println("A  = " + A);
        System.out.println("B  = " + B);

    }
}
