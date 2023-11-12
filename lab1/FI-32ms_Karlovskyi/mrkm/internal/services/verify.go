package services

import (
	"context"
	"crypto"
	"crypto/rsa"
	"crypto/sha512"
	"errors"
	"mrkm/internal/entities"
	"mrkm/internal/errs"
	"mrkm/internal/repositories"
)

type VerifyService struct {
	userRepository repositories.UserRepository
}

func NewVerifyService(userRepository repositories.UserRepository) *VerifyService {
	return &VerifyService{userRepository: userRepository}
}

func (s *VerifyService) Register(ctx context.Context, nickname string, publicKey []byte) error {
	_, err := s.userRepository.FindBy(ctx, publicKey)
	if err == nil {
		return errs.ErrAccountAlreadyExists
	}

	if !errors.Is(err, errs.ErrAccountDoesNotExists) {
		return err
	}

	u := &entities.User{
		Nickname: nickname,

		PublicKey: publicKey,
	}

	if _, err := s.userRepository.Create(ctx, u); err != nil {
		return err
	}

	return nil
}

func (s *VerifyService) Verify(ctx context.Context, content, signature, publicKey []byte) (*entities.User, bool, error) {
	key := entities.ParsePublicKey(publicKey)

	contentHash := sha512.New()
	if _, err := contentHash.Write(content); err != nil {
		return nil, false, err
	}

	contentHashSum := contentHash.Sum(nil)

	user, err := s.userRepository.FindBy(ctx, publicKey)
	if errors.Is(err, errs.ErrAccountDoesNotExists) {
		return nil, false, rsa.VerifyPSS(key, crypto.SHA512, contentHashSum, signature, nil)
	}

	if err != nil {
		return nil, false, err
	}

	return user, true, rsa.VerifyPSS(key, crypto.SHA512, contentHashSum, signature, nil)
}
