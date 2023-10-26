package services

import (
	"context"
	"crypto"
	"crypto/rand"
	"crypto/rsa"
	"crypto/sha512"
	"golang.org/x/crypto/bcrypt"
	"mrkm/internal/entities"
	"mrkm/internal/errs"
	"mrkm/internal/repositories"
)

type SignService struct {
	userRepository repositories.UserRepository
}

func NewSignService(userRepository repositories.UserRepository) *SignService {
	return &SignService{userRepository: userRepository}
}

func (s *SignService) Register(ctx context.Context, login, password string) (*entities.UserClientView, error) {
	_, err := s.userRepository.FindBy(ctx, map[string]interface{}{"login": login})
	if err == nil {
		return nil, errs.ErrAccountAlreadyExists
	}

	passwordHash, err := bcrypt.GenerateFromPassword([]byte(password), 15)
	if err != nil {
		return nil, err
	}

	privateKey, err := rsa.GenerateKey(rand.Reader, 4096)
	if err != nil {
		return nil, err
	}

	u := &entities.User{
		Login:        login,
		PasswordHash: string(passwordHash),

		Key: privateKey,
	}

	if _, err := s.userRepository.Create(ctx, u); err != nil {
		return nil, err
	}

	return u.ToClientView(), nil
}

func (s *SignService) Sign(ctx context.Context, content, privateKey []byte) (string, error) {
	key, err := entities.ParsePrivateKey(string(privateKey))
	if err != nil {
		return "", err
	}

	contentHash := sha512.New()
	if _, err = contentHash.Write(content); err != nil {
		return "", err
	}

	contentHashSum := contentHash.Sum(nil)

	signature, err := rsa.SignPSS(rand.Reader, key, crypto.SHA512, contentHashSum, nil)
	if err != nil {
		return "", err
	}

	return string(signature), nil
}

func (s *SignService) Verify(ctx context.Context, content, signature, publicKey []byte) error {
	key, err := entities.ParsePublicKey(string(publicKey))
	if err != nil {
		return err
	}

	contentHash := sha512.New()
	if _, err = contentHash.Write(content); err != nil {
		return err
	}

	contentHashSum := contentHash.Sum(nil)

	return rsa.VerifyPSS(key, crypto.SHA512, contentHashSum, signature, nil)
}
