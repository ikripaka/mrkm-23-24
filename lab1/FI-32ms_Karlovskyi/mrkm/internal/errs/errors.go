package errs

import "errors"

var (
	ErrAccountDoesNotExists = errors.New("account does not exists")
	ErrAccountAlreadyExists = errors.New("account with provided credentials already exists")
)
