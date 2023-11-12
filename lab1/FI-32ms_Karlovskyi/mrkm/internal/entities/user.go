package entities

import (
	"bytes"
	"crypto/rsa"
	"fmt"
	"math/big"
)

type User struct {
	Nickname string

	PublicKey []byte `json:"type:bytea"`
}

func FormatKey(k *rsa.PrivateKey) (pub, priv []byte) {
	priv = []byte(k.D.String())

	for _, p := range k.Primes {
		priv = append(priv, '_')
		priv = append(priv, []byte(p.String())...)
	}

	pub = k.PublicKey.N.Bytes()

	return pub, priv
}

func ParsePrivateKey(key []byte) (*rsa.PrivateKey, error) {
	split := bytes.Split(key, []byte{'_'})
	if len(split) != 3 {
		return nil, fmt.Errorf("wrong key")
	}

	d, p1, p2 := new(big.Int), new(big.Int), new(big.Int)
	d, _ = d.SetString(string(split[0]), 10)
	p1, _ = p1.SetString(string(split[1]), 10)
	p2, _ = p2.SetString(string(split[2]), 10)

	privateKey := &rsa.PrivateKey{
		D:      d,
		Primes: []*big.Int{p1, p2},
		PublicKey: rsa.PublicKey{
			N: new(big.Int).Mul(p1, p2),
			E: 65537,
		},
	}
	if err := privateKey.Validate(); err != nil {
		return nil, err
	}

	privateKey.Precompute()

	return privateKey, nil
}

func ParsePublicKey(key []byte) *rsa.PublicKey {
	n, e := new(big.Int), 65537
	n = n.SetBytes(key)

	return &rsa.PublicKey{
		N: n,
		E: e,
	}
}
