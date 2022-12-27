## [1.4.6](https://github.com/vtavernier/utc-telegram-bot/compare/v1.4.5...v1.4.6) (2022-12-27)


### Bug Fixes

* **deps:** update rust crate sqlx to 0.6 ([82b7489](https://github.com/vtavernier/utc-telegram-bot/commit/82b7489e79f11807340e9184b0ceeb46085429cf))
* **deps:** update rust crate tokio to 1.21 ([42a0b73](https://github.com/vtavernier/utc-telegram-bot/commit/42a0b731bd8f090131a20a3989acf6bda87f497c))
* **deps:** update rust crate tokio to 1.22 ([6d6ce1a](https://github.com/vtavernier/utc-telegram-bot/commit/6d6ce1a5b9fbfa9d9ef3cf2c541feb9e84e50bd8))
* **deps:** update rust crate tokio to 1.23 ([2b18233](https://github.com/vtavernier/utc-telegram-bot/commit/2b182335a4ba1c116b54fb3b7d0195f2cac7e72d))

## [1.4.5](https://github.com/vtavernier/utc-telegram-bot/compare/v1.4.4...v1.4.5) (2022-11-16)


### Dependencies and Other Build Updates

* **deps:** bump futures from 0.3.19 to 0.3.25 ([ab9ce96](https://github.com/vtavernier/utc-telegram-bot/commit/ab9ce964ee69f48f46ea053deb37c63914e3bb71))
* **deps:** bump rand from 0.8.4 to 0.8.5 ([e8b5243](https://github.com/vtavernier/utc-telegram-bot/commit/e8b524331dce162349801dab809a0212473c77e0))
* **deps:** bump serde from 1.0.133 to 1.0.147 ([833e6a1](https://github.com/vtavernier/utc-telegram-bot/commit/833e6a14631d50d48d16272c8e85fa5af62491dd))
* **deps:** bump tracing from 0.1.29 to 0.1.34 ([ae43e92](https://github.com/vtavernier/utc-telegram-bot/commit/ae43e923093523c50402df5c19dd2ee6019b894a))

## [1.4.4](https://github.com/vtavernier/utc-telegram-bot/compare/v1.4.3...v1.4.4) (2022-11-10)


### Bug Fixes

* **docker:** add missing image metadata ([#23](https://github.com/vtavernier/utc-telegram-bot/issues/23)) ([4ba913e](https://github.com/vtavernier/utc-telegram-bot/commit/4ba913e0f2bbd0cb1f1428259d31d816d95678a0))

## [1.4.3](https://github.com/vtavernier/utc-telegram-bot/compare/v1.4.2...v1.4.3) (2022-11-10)


### Dependencies and Other Build Updates

* **deps:** bump chrono from 0.4.19 to 0.4.22 ([c0e9b70](https://github.com/vtavernier/utc-telegram-bot/commit/c0e9b708e2691aa4570f9d0ceac341cf41a5fc6c))
* **deps:** bump crc from 2.1.0 to 3.0.0 ([3e467fb](https://github.com/vtavernier/utc-telegram-bot/commit/3e467fb34764e80371ad36b42e02268b8860ad8f))
* **deps:** bump serde_json from 1.0.74 to 1.0.87 ([19683dc](https://github.com/vtavernier/utc-telegram-bot/commit/19683dc5b5d80976d9606b6e038b3e9c8f4a62fa))
* **deps:** bump thiserror from 1.0.30 to 1.0.37 ([c3cd2f1](https://github.com/vtavernier/utc-telegram-bot/commit/c3cd2f1678367cea66b05c326762d2b7a8a1ef42))
* **deps:** bump tokio from 1.15.0 to 1.16.1 ([f9ff5ac](https://github.com/vtavernier/utc-telegram-bot/commit/f9ff5acaa0e2d61874029ef1ca0fe2520ff4c5ec))

## [1.4.2](https://github.com/vtavernier/utc-telegram-bot/compare/v1.4.1...v1.4.2) (2022-11-03)


### Bug Fixes

* **docker:** ensure cargo build actually builds the binary ([5779937](https://github.com/vtavernier/utc-telegram-bot/commit/5779937a968f2505cce662da07749c4181dd0cf2))

## [1.4.1](https://github.com/vtavernier/utc-telegram-bot/compare/v1.4.0...v1.4.1) (2022-11-01)


### Dependencies and Other Build Updates

* **deps:** bump cairo-rs from 0.14.9 to 0.16.1 ([171fcd8](https://github.com/vtavernier/utc-telegram-bot/commit/171fcd8ac4cca27d5defdb1746970d5b42e74bba))
* **deps:** bump structopt from 0.3.25 to 0.3.26 ([6bb0ada](https://github.com/vtavernier/utc-telegram-bot/commit/6bb0adad2cc58172e59f3d607237a170a2829282))

## [1.4.0](https://github.com/vtavernier/utc-telegram-bot/compare/v1.3.0...v1.4.0) (2022-10-31)


### Features

* create and migrate database on start ([0654835](https://github.com/vtavernier/utc-telegram-bot/commit/0654835743bcb0aadae9faaef047fc3e1b5e8dad))

## [1.3.0](https://github.com/vtavernier/utc-telegram-bot/compare/v1.2.0...v1.3.0) (2022-10-31)


### Features

* move server functionality to run feature ([545a841](https://github.com/vtavernier/utc-telegram-bot/commit/545a841cd96763e48f5913009222b0b50bcedcf2))


### Bug Fixes

* **converter:** fix clippy warnings ([8f51a77](https://github.com/vtavernier/utc-telegram-bot/commit/8f51a7719f3144e9dacd641d4acaa6a744957d74))
* **run:** fix clippy warnings ([ff8fc2b](https://github.com/vtavernier/utc-telegram-bot/commit/ff8fc2bb71fa8ea9df03703ed0f0b0631d546c55))


### Dependencies and Other Build Updates

* **docker:** cache dependencies in separate layer ([d6c8614](https://github.com/vtavernier/utc-telegram-bot/commit/d6c8614ac0a12407daffb9d575e2c28f0a03daf8))
* set explicit distroless/cc version ([5c5eadb](https://github.com/vtavernier/utc-telegram-bot/commit/5c5eadb864b3f173606024deec879a729e09b43f))
* set rust-toolchain version to 1.63.0 ([7cbb0a3](https://github.com/vtavernier/utc-telegram-bot/commit/7cbb0a3535f391210f318e4168d544b6186bb2e3))
* support dependabot updates ([ac8d7ec](https://github.com/vtavernier/utc-telegram-bot/commit/ac8d7ecb78de301563f6bff0b6b5ef160335b962))

# [1.2.0](https://github.com/vtavernier/utc-telegram-bot/compare/v1.1.3...v1.2.0) (2022-02-13)


### Features

* add image manifest and cache invalidation hash ([4b4e22b](https://github.com/vtavernier/utc-telegram-bot/commit/4b4e22b58998b5b6406ea5c9c4ef8d087bad5d3f))

## [1.1.3](https://github.com/vtavernier/utc-telegram-bot/compare/v1.1.2...v1.1.3) (2022-02-12)


### Bug Fixes

* add extra fonts in CI builds ([3c6d80d](https://github.com/vtavernier/utc-telegram-bot/commit/3c6d80d17efb42ccadb5c750f662f73a759b7c76))

## [1.1.2](https://github.com/vtavernier/utc-telegram-bot/compare/v1.1.1...v1.1.2) (2022-02-11)


### Bug Fixes

* add fonts-dejavu in CI builds ([7ad8de7](https://github.com/vtavernier/utc-telegram-bot/commit/7ad8de7b8a9cb012ab558c40ac6ea525a5cdf3c5))

## [1.1.1](https://github.com/vtavernier/utc-telegram-bot/compare/v1.1.0...v1.1.1) (2022-01-23)


### Bug Fixes

* terminate gracefully on SIGTERM ([fec0acd](https://github.com/vtavernier/utc-telegram-bot/commit/fec0acd0aac7f8ead72043a7744dd7049f114ae1))

# [1.1.0](https://github.com/vtavernier/utc-telegram-bot/compare/v1.0.0...v1.1.0) (2022-01-22)


### Bug Fixes

* add default argument values ([71eacf1](https://github.com/vtavernier/utc-telegram-bot/commit/71eacf1070dee57c8ac25858c1765ba4e7f6298b))
* exit on invalid token ([087b3ee](https://github.com/vtavernier/utc-telegram-bot/commit/087b3eedfbe168378c03d733e55e92e93e895d80))
* fix image generation ([869559e](https://github.com/vtavernier/utc-telegram-bot/commit/869559e72ab3da4d4ad206ca73b688c5834608e7))


### Features

* generate images in public/images ([36dda5c](https://github.com/vtavernier/utc-telegram-bot/commit/36dda5caf651c63cdf8dddc4c6d202c03fe51589))
* serve transform images ([75f9c9e](https://github.com/vtavernier/utc-telegram-bot/commit/75f9c9ea1717fd27f0b1784f1fc8b37c09c054c5))

# 1.0.0 (2022-01-09)


### Features

* initial release ([2d17d38](https://github.com/vtavernier/utc-telegram-bot/commit/2d17d3867707600a801b8d874b62789e5e599d65))
