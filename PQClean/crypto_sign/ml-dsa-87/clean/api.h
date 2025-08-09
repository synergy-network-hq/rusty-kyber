#ifndef PQCLEAN_MLDSA87_CLEAN_API_H
#define PQCLEAN_MLDSA87_CLEAN_API_H

#include <stddef.h>
#include <stdint.h>

#define PQCLEAN_MLDSA87_CLEAN_CRYPTO_PUBLICKEYBYTES 2592
#define PQCLEAN_MLDSA87_CLEAN_CRYPTO_SECRETKEYBYTES 4896
#define PQCLEAN_MLDSA87_CLEAN_CRYPTO_BYTES 4627
#define PQCLEAN_MLDSA87_CLEAN_CRYPTO_ALGNAME "ML-DSA-87"

int PQCLEAN_MLDSA87_CLEAN_crypto_sign_keypair(uint8_t *pk, uint8_t *sk);

int PQCLEAN_MLDSA87_CLEAN_crypto_sign_signature_ctx(uint8_t *sig, size_t *siglen,
        const uint8_t *m, size_t mlen,
        const uint8_t *ctx, size_t ctxlen,
        const uint8_t *sk);

int PQCLEAN_MLDSA87_CLEAN_crypto_sign_ctx(uint8_t *sm, size_t *smlen,
        const uint8_t *m, size_t mlen,
        const uint8_t *ctx, size_t ctxlen,
        const uint8_t *sk);

int PQCLEAN_MLDSA87_CLEAN_crypto_sign_verify_ctx(const uint8_t *sig, size_t siglen,
        const uint8_t *m, size_t mlen,
        const uint8_t *ctx, size_t ctxlen,
        const uint8_t *pk);

int PQCLEAN_MLDSA87_CLEAN_crypto_sign_open_ctx(uint8_t *m, size_t *mlen,
        const uint8_t *sm, size_t smlen,
        const uint8_t *ctx, size_t ctxlen,
        const uint8_t *pk);

int PQCLEAN_MLDSA87_CLEAN_crypto_sign_signature(uint8_t *sig, size_t *siglen,
        const uint8_t *m, size_t mlen,
        const uint8_t *sk);

int PQCLEAN_MLDSA87_CLEAN_crypto_sign(uint8_t *sm, size_t *smlen,
                                      const uint8_t *m, size_t mlen,
                                      const uint8_t *sk);

int PQCLEAN_MLDSA87_CLEAN_crypto_sign_verify(const uint8_t *sig, size_t siglen,
        const uint8_t *m, size_t mlen,
        const uint8_t *pk);

int PQCLEAN_MLDSA87_CLEAN_crypto_sign_open(uint8_t *m, size_t *mlen,
        const uint8_t *sm, size_t smlen,
        const uint8_t *pk);

#endif
