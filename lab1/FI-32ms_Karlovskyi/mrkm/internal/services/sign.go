package services

import (
	"context"
	"crypto"
	"crypto/rand"
	"crypto/rsa"
	"crypto/sha512"
	"mrkm/internal/entities"
)

type SignService struct{}

func NewSignService() *SignService {
	return &SignService{}
}

func (s *SignService) Sign(ctx context.Context, content, privateKey []byte) ([]byte, error) {
	key, err := entities.ParsePrivateKey(privateKey)
	if err != nil {
		return nil, err
	}

	contentHash := sha512.New()
	if _, err = contentHash.Write(content); err != nil {
		return nil, err
	}

	contentHashSum := contentHash.Sum(nil)

	signature, err := rsa.SignPSS(rand.Reader, key, crypto.SHA512, contentHashSum, nil)
	if err != nil {
		return nil, err
	}

	return signature, nil
}

func (s *SignService) Generate(ctx context.Context) (*rsa.PrivateKey, error) {
	return rsa.GenerateKey(rand.Reader, 4096)
}
