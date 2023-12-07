import java.math.BigInteger;
import java.util.ArrayList;
import java.util.Objects;

public class crypto {
    public static int bitsNum = 1024;
    public static int iter = 3;

    private static boolean gcd(BigInteger a, BigInteger b) {
        if (a.compareTo(b) < 0) {
            BigInteger temp = a;
            a = b;
            b = temp;
        }
        BigInteger r = BigInteger.valueOf(1);
        while (b.compareTo(BigInteger.valueOf(1)) != 0) {
            r = a.remainder(b);
            a = b;
            b = r;
            if (b.compareTo(BigInteger.valueOf(0)) == 0) {
                b = BigInteger.valueOf(1);
                r = a;
            }
        }
        return r.equals(BigInteger.valueOf(1));
    }

    private static boolean tryToDivide(BigInteger a) {
        ArrayList<BigInteger> divides = new ArrayList<>();
        divides.add(BigInteger.valueOf(2));
        for (int i = 1; i < 3; i++) {
            divides.add(divides.get(i - 1).add(BigInteger.valueOf(1)));
        }
        for (BigInteger divide : divides) {
            if (a.remainder(divide).compareTo(BigInteger.valueOf(0)) == 0) return false;
        }
        return true;
    }

    private static int MillerRabin(BigInteger n) {
        int s = 0;
        int k = iter;
        BigInteger x;
        BigInteger d = n.subtract(BigInteger.valueOf(1));
        while ((d.remainder(BigInteger.valueOf(2))).compareTo(BigInteger.valueOf(0)) == 0 && d.compareTo(BigInteger.valueOf(0)) != 0) {
            d = d.divide(BigInteger.valueOf(2));
            s++;
        }
        for (int j = 0; j < k; j++) {
            int xTemp = (int) ((Math.random() + 1) * 1_000_000);
            x = new BigInteger(String.valueOf(xTemp), 10);
            if (!gcd(x, n)) {
                return 0;
            }
            BigInteger x0 = x.modPow(d, n);
            if (x0.compareTo(BigInteger.valueOf(1)) == 0 || x0.compareTo(n.subtract(BigInteger.valueOf(1))) == 0) {
                continue;
            } else {
                for (int i = 0; i < s; i++) {
                    x0 = x0.modPow(BigInteger.valueOf(2), n);
                    if (x0.compareTo(n.subtract(BigInteger.valueOf(1))) == 0) break;
                    else if (i == s - 1 && x0.compareTo(BigInteger.valueOf(1)) != 0 && x0.compareTo(n.subtract(BigInteger.valueOf(1))) != 0)
                        return 0;
                }
            }
        }
        return 1;
    }

    private static int Jacobi(BigInteger a, BigInteger n) {
        if (!Objects.equals(gcd(a, n), true))
            return 0;
        int r = 1;
        if (a.signum() == -1) a = a.negate();
        BigInteger d = a;
        while (!d.equals(BigInteger.valueOf(0))) {
            int t = 0;
            while (d.remainder(BigInteger.valueOf(2)).equals(
                    BigInteger.valueOf(0)) && d.compareTo(BigInteger.valueOf(0))
                    > 0) {
                d = d.divide(BigInteger.valueOf(2));
                t++;
            }
            if (t % 2 == 1) {
                if (n.mod(BigInteger.valueOf(8)).equals(
                        BigInteger.valueOf(5)) || n.mod(BigInteger.valueOf(8)).
                        equals(BigInteger.valueOf(3)))
                    r = -r;
            }
            if (d.mod(BigInteger.valueOf(4)).equals(n.mod(
                    BigInteger.valueOf(4))) && n.mod(BigInteger.valueOf(4)).
                    equals(BigInteger.valueOf(3)))
                r = -r;
            BigInteger b = d;
            d = n.mod(b);
            n = b;
        }
        return r;
    }

    private static int verifyMillerRabin(BigInteger n) {
        int s = 0;
        int k = 10000;
        BigInteger x;
        BigInteger d = n.subtract(BigInteger.valueOf(1));
        while ((d.remainder(BigInteger.valueOf(2))).compareTo(BigInteger.valueOf(0)) == 0 && d.compareTo(BigInteger.valueOf(0)) != 0) {
            d = d.divide(BigInteger.valueOf(2));
            s++;
        }
        for (int j = 0; j < k; j++) {
            int xTemp = (int) ((Math.random() + 1) * 1_000_000);
            x = new BigInteger(String.valueOf(xTemp), 10);
            if (!gcd(x, n)) {
                return 0;
            }
            BigInteger x0 = x.modPow(d, n);
            if (x0.compareTo(BigInteger.valueOf(1)) == 0 || x0.compareTo(n.subtract(BigInteger.valueOf(1))) == 0) {
                continue;
            }
            for (int i = 0; i < s; i++) {
                x0 = x0.modPow(BigInteger.valueOf(2), n);
                if (x0.compareTo(n.subtract(BigInteger.valueOf(1))) == 0) break;
                else if (i == s - 1 && x0.compareTo(BigInteger.valueOf(1)) != 0 && x0.compareTo(n.subtract(BigInteger.valueOf(1))) != 0)
                    return 0;
            }
        }
        return 1;
    }

    private static int Fermat(BigInteger n) {
        int k = iter;
        BigInteger x;
        BigInteger d = n.subtract(BigInteger.valueOf(1));
        for (int j = 0; j < k; j++) {
            int xTemp = (int) ((Math.random() + 1) * 1_000_000);
            x = new BigInteger(String.valueOf(xTemp), 10);
            if (!gcd(x, n)) {
                return 0;
            }
            BigInteger x0 = x.modPow(d, n);
            if (x0.compareTo(BigInteger.valueOf(1)) != 0) return 0;
        }
        return 1;
    }

    private static int SolovayStrassen(BigInteger n) {
        int k = iter;
        BigInteger x;
        BigInteger d = n.subtract(BigInteger.valueOf(1));
        d = d.divide(BigInteger.valueOf(2));
        for (int j = 0; j < k; j++) {
            int xTemp = (int) ((Math.random() + 1) * 1_000_000);
            x = new BigInteger(String.valueOf(xTemp), 10);
            if (!gcd(x, n)) {
                return 0;
            }
            BigInteger x0 = x.modPow(d, n);
            if (x0.compareTo((BigInteger.valueOf(Jacobi(x, n))).mod(n)) != 0) return 0;
        }
        return 1;
    }


    public static String BlumBlumaBits(BigInteger r1) {
        StringBuilder blbl = new StringBuilder();
        StringBuilder temp = new StringBuilder();
        BigInteger P = new BigInteger("D5BBB96D30086EC484EBA3D7F9CAEB07", 16);
        BigInteger Q = new BigInteger("425D2B9BFDB25B9CF6C416CC6E37B59C1F", 16);
        BigInteger n = P.multiply(Q);
        BigInteger two = new BigInteger("2");
        BigInteger r2;
        while (blbl.length() != bitsNum) {
            r2 = r1.modPow(two, n);
            String last = r2.toString(2);
            temp.append(last.charAt(last.length() - 1));
            if (temp.length() == 8) {
                blbl.append(temp);
                temp = new StringBuilder();
            }
            r1 = r2;
        }
        return blbl.toString();
    }


    public static void main(String[] args) throws Exception {
        BigInteger A = new BigInteger("5B88C41246790891C095E2878880342E88C79974303BD0400B090FE38A688356", 16);
        BigInteger B = new BigInteger("675215CC3E227D3216C056CFA8F8822BB486F788641E85E0DE77097E1DB049F1", 16);
        BigInteger P = new BigInteger("CEA42B987C44FA642D80AD9F51F10457690DEF10C83D0BC1BCEE12FC3B6093E3", 16);
        int k = 0;
        int n1 = 0, n2 = 0, n3 = 0, n4 = 0;
        long t2 = 0, t4 = 0, t6 = 0;
        BigInteger r1 = new BigInteger("BCEF", 16);
        boolean div = false;
        while (k != 800000) {
            BigInteger N = new BigInteger("0", 2);
            while (!div) {
                String a = BlumBlumaBits(r1);
                N = new BigInteger(a, 2);
                if (tryToDivide(N)) div = true;
                r1 = r1.add(BigInteger.valueOf(1));
            }
            div = false;
            long t1 = System.nanoTime();
            n1 += Fermat(N);
            t2 = System.nanoTime() - t1 + t2;
            long t3 = System.nanoTime();
            n2 += MillerRabin(N);
            t4 = System.nanoTime() - t3 + t4;
            long t5 = System.nanoTime();
            n3 += SolovayStrassen(N);
            t6 = System.nanoTime() - t5 + t6;
            n4 += verifyMillerRabin(N);
            k++;
            r1 = r1.add(BigInteger.valueOf(1));
            if (k % 100000 == 0) System.out.println(k);
        }

        System.out.println("Results of test:");
        System.out.println("Fermat method:" + n1 + "/100, while the arguments (numerators) are " + iter);
        System.out.println("Solovay-Strassen method:" + n3 + "/100, while the arguments (numerators) are " + iter);
        System.out.println("Miller-Rabin method:" + n2 + "/100, while the arguments (numerators) are " + iter);
        System.out.println("The right amount of prime numbers is " + n4);
        System.out.println("________________________________________________________________________________________");
        System.out.println("Iterations time averaged value:");
        System.out.println("Fermat:" + t2 / 100000);
        System.out.println("Solovay-Strassen:" + t6 / 100000);
        System.out.println("Miller-Rabin:" + t4 / 100000);
    }
}
