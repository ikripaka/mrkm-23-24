package entities

import (
	"crypto/rsa"
	"fmt"
	"math/big"
	"strconv"
	"strings"
)

type User struct {
	Login        string
	PasswordHash string

	Key *rsa.PrivateKey `gorm:"serializer:json"`
}

func (u *User) ToClientView() *UserClientView {
	pub, priv := FormatKey(u.Key)

	return &UserClientView{
		Login:      u.Login,
		PublicKey:  pub,
		PrivateKey: priv,
	}
}

func FormatKey(k *rsa.PrivateKey) (pub, priv string) {
	priv = fmt.Sprintf("%v", k.D)

	for _, p := range k.Primes {
		priv += fmt.Sprintf("_%v", p)
	}

	return fmt.Sprintf("%v_%v", k.PublicKey.N, k.PublicKey.E), priv
}

func ParsePrivateKey(key string) (*rsa.PrivateKey, error) {
	split := strings.Split(key, "_")
	if len(split) != 3 {
		return nil, fmt.Errorf("wrong key")
	}

	d, p1, p2 := new(big.Int), new(big.Int), new(big.Int)
	d, _ = d.SetString(split[0], 10)
	p1, _ = p1.SetString(split[1], 10)
	p2, _ = p2.SetString(split[2], 10)

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

type UserClientView struct {
	Login string `json:"login"`

	PublicKey  string `json:"public_key"`
	PrivateKey string `json:"private_key"`
}

func ParsePublicKey(key string) (*rsa.PublicKey, error) {
	split := strings.Split(key, "_")
	if len(split) != 2 {
		return nil, fmt.Errorf("wrong key")
	}

	n, e := new(big.Int), 0
	n, _ = n.SetString(split[0], 10)

	var err error

	e, err = strconv.Atoi(split[1])
	if err != nil {
		return nil, err
	}

	return &rsa.PublicKey{
		N: n,
		E: e,
	}, nil
}
