import java.math.BigInteger;
import java.util.ArrayList;

public class BlindSignature {
    public static int bitsNum = 1024;
    public static BigInteger e = new BigInteger("10001", 16);

    public static String BlumBlumaBits(BigInteger r1, int bits) {
        StringBuilder blbl = new StringBuilder();
        StringBuilder temp = new StringBuilder();
        BigInteger P = new BigInteger("D5BBB96D30086EC484EBA3D7F9CAEB07", 16);
        BigInteger Q = new BigInteger("425D2B9BFDB25B9CF6C416CC6E37B59C1F", 16);
        BigInteger n = P.multiply(Q);
        BigInteger two = new BigInteger("2");
        BigInteger r2;
        while (blbl.length() != bits) {
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
    private static int MillerRabin(BigInteger n) {
        int s = 0;
        int k = 10;
        BigInteger x;
        BigInteger d = n.subtract(BigInteger.valueOf(1));
        while ((d.remainder(BigInteger.valueOf(2))).compareTo(BigInteger.valueOf(0)) == 0 && d.compareTo(BigInteger.valueOf(0)) != 0) {
            d = d.divide(BigInteger.valueOf(2));
            s++;
        }
        for (int j = 0; j < k; j++) {
            int xTemp = (int) ((Math.random() + 1) * 1_000_000);
            x = new BigInteger(String.valueOf(xTemp), 10);

            if (!x.gcd(n).equals(BigInteger.valueOf(1))) {
                return 0;
            }
            BigInteger x0 = x.modPow(d, n);
            if (x0.compareTo(BigInteger.valueOf(1)) == 0 || x0.compareTo(n.subtract(BigInteger.valueOf(1))) == 0) {
                continue;
            }
            else {
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

    private static BigInteger EuclidsAlgorithm(BigInteger a, BigInteger b) {
        BigInteger u2, v2, r, q;
        BigInteger rev_a = BigInteger.valueOf(0);
        BigInteger u0 = BigInteger.valueOf(1);
        BigInteger v0 = BigInteger.valueOf(0);
        BigInteger u1 = BigInteger.valueOf(0);
        BigInteger v1 = BigInteger.valueOf(1);
        BigInteger n = b;
        while (b.compareTo(BigInteger.valueOf(1)) != 0) {
            q = a.divide(b);
            r = a.remainder(b);
            a = b;
            b = r;
            if (b.compareTo(BigInteger.valueOf(0)) == 0) {
                b = BigInteger.valueOf(1);
            }
            u2 = u0.subtract(q.multiply(u1));
            v2 = v0.subtract(q.multiply(v1));
            u0 = u1;
            v0 = v1;
            u1 = u2;
            v1 = v2;
            rev_a = u2;
        }
        rev_a = rev_a.mod(n);
        return rev_a;
    }
    public static BigInteger power(BigInteger N, BigInteger pow){
        BigInteger staticN = N;
        String BitPow = pow.toString(2);
        for(int i=0; i<BitPow.length(); i++){
            N = N.pow(2);
            if(BitPow.charAt(i) == '1'){
                N=N.multiply(staticN);
            }
        }
        return N;
    }

    public static BigInteger Ith_Root(BigInteger N, BigInteger K) {

        BigInteger K1 = K.subtract(BigInteger.ONE);
        BigInteger S  = N.add(BigInteger.ONE);
        BigInteger U  = N;
        while (U.compareTo(S) < 0) {
            S = U;
            U = power(U, K1);
            N = N.divide(U);
            BigInteger Utemp = U.multiply(K1);
            Utemp = Utemp.add(U);
            U=Utemp.divide(K);
        }
        return S;
    }

    private static BigInteger MessageMaskForm(BigInteger r, BigInteger message, BigInteger mod){
        BigInteger mnew = r.modPow(e,mod);
        mnew = mnew.multiply(message);
        return mnew;
    }

    private static BigInteger Signature(BigInteger d, BigInteger mod, BigInteger mnew)
    {
        mnew = mnew.modPow(d,mod);
        return mnew;
    }

    private static BigInteger MaskClear(BigInteger r, BigInteger s, BigInteger mod)
    {
        r = r.modInverse(mod);
        s = s.multiply(r).mod(mod);
        return s;
    }

    private static ArrayList<BigInteger> generationRSAKey(BigInteger r1) {
        boolean div = false;
        BigInteger p = new BigInteger("1", 10);
        BigInteger q = new BigInteger("1", 10);
        while (!div) {
            String a = BlumBlumaBits(r1, bitsNum);
            p = new BigInteger(a, 2);
            if (MillerRabin(p) == 1) div = true;
            r1 = r1.add(BigInteger.valueOf(1));
        }
        while (div) {
            String a = BlumBlumaBits(r1, bitsNum);
            q = new BigInteger(a, 2);
            if (MillerRabin(q) == 1) div = false;
            r1 = r1.add(BigInteger.valueOf(1));
        }
        ArrayList<BigInteger> pAndq = new ArrayList<>();
        pAndq.add(p);
        pAndq.add(q);
    return pAndq;
    }

    private static void RSABlindSignature(BigInteger r1, BigInteger message) {
        ArrayList<BigInteger> pAndq = generationRSAKey(r1);
        BigInteger p = pAndq.get(0);
        BigInteger q = pAndq.get(1);
        BigInteger n = q.multiply(p);
        System.out.println(n.toString(16));
        BigInteger fi_n = (p.subtract(BigInteger.valueOf(1))).multiply(q.subtract(BigInteger.valueOf(1)));
        BigInteger d = EuclidsAlgorithm(e, fi_n);
        //System.out.println(n.toString(16));
        String rmask = BlumBlumaBits(r1.subtract(BigInteger.valueOf(15)), 40);
        BigInteger r = new BigInteger(rmask, 2);
        System.out.println("This is a Blind signature RSA test:");
        System.out.println("m` value with mask = ");
        BigInteger mnew = MessageMaskForm(r, message, n);
        System.out.println(mnew.toString(16));
        System.out.println("s` value = ");
        BigInteger snew = Signature(d, n, mnew);
        System.out.println(snew.toString(16));
        System.out.println("Clear mask, s = ");
        System.out.println(MaskClear(r, snew, n).toString(16));
        System.out.println("Check equal with normal signature value");
        System.out.println(message.modPow(d, n).toString(16));
    }


    private static void generationSchnorrKey(BigInteger r1){
        boolean div = false;
        BigInteger p =new BigInteger("1",10);
        BigInteger q1 =new BigInteger("1",10);
        BigInteger q2 =new BigInteger("1",10);
        BigInteger g =new BigInteger("1",10);
        while (!div)
        {
            String a = BlumBlumaBits(r1, 24);
            q1 = new BigInteger(a,2);
            if(MillerRabin(q1) == 1) div = true;
            r1 = r1.add(BigInteger.valueOf(1));
        }
        while (div)
        {
            String a = BlumBlumaBits(r1, 24);
            q2 = new BigInteger(a,2);
            p=(q1.multiply(q2)).add(BigInteger.ONE);
            if(MillerRabin(p) == 1) div = false;
            r1 = r1.add(BigInteger.valueOf(1));
        }
        BigInteger gTemp =new BigInteger("1",10);
        while (gTemp.equals(BigInteger.ONE)) {
            gTemp = Ith_Root(p.add(BigInteger.ONE), q1);
            p = (p.pow(2)).add(BigInteger.ONE);
        }
        g = gTemp;
    }




    public static void main(String[] args) throws Exception {
        BigInteger r1 = new BigInteger("BCEF", 16);
        BigInteger message = new BigInteger("ABCDEDD", 16);
        RSABlindSignature(r1, message);
        generationSchnorrKey(r1);
    }
}
