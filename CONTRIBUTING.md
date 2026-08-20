# Участие в разработке Oreluno

Спасибо за интерес к Oreluno.

Перед отправкой изменений ознакомьтесь с правилами ниже. Они нужны не только для качества кода, но и для того, чтобы права на принятые изменения оставались понятными для автора вклада и проекта.

## Перед отправкой изменений

Отправляя вклад в Oreluno, убедитесь, что:

- вы являетесь автором кода или имеете достаточные права на его передачу проекту
- вклад не нарушает права работодателя, заказчика или другой организации
- сторонний код и материалы не копируются в проект без совместимой лицензии и необходимых разрешений
- код, созданный с помощью автоматических средств или генеративных систем, был вами внимательно проверен на корректность, безопасность и соответствие требованиям проекта
- вы готовы принять действующую версию `CLA.md` через CLA Assistant перед объединением Pull Request

Если вы вносите код от имени компании или другой организации, сначала свяжитесь с владельцем проекта: для таких вкладов может потребоваться отдельное соглашение с организацией.

## Лицензирование вклада

Авторское право на ваш собственный вклад остаётся у вас.

При принятии CLA вы предоставляете владельцу Oreluno дополнительные права, необходимые для использования, изменения, распространения, коммерческого лицензирования, сублицензирования и перелицензирования принятого вклада.

Если вклад включён в публичную версию Oreluno, он также остаётся доступен на условиях публичной лицензии, действовавшей для Oreluno на дату его отправки.

Подробные юридические условия определяются файлом [`CLA.md`](CLA.md).

Публичное использование Oreluno регулируется файлом [`LICENSE.md`](LICENSE.md). Информация о коммерческом лицензировании находится в [`COMMERCIAL.md`](COMMERCIAL.md).

## Подготовка изменений

Перед Pull Request выполните:

```powershell
cargo fmt
cargo check
cargo clippy
cargo test
```

Все команды должны завершаться без ошибок. `cargo clippy` не должен добавлять новые предупреждения без обоснованной причины.

Если изменение добавляет новое поведение или исправляет ошибку, по возможности добавьте тест, который подтверждает ожидаемый результат.

## Код и документация

Для проекта предпочтительны:

- понятные имена и явные контракты
- отсутствие необъяснённых числовых констант и коэффициентов
- краткое объяснение происхождения необычных значений и формул
- комментарии и документация могут быть на русском или английском языке без обязательного приоритета одного из них
- публичная документация API, включая будущую документацию `cargo doc`, может быть на русском, английском или обоих языках
- общепринятые технические имена API и терминов следует сохранять в точной и однозначной форме
- минимальные зависимости: новая зависимость должна иметь понятную необходимость

Не выполняйте крупный архитектурный рефакторинг вместе с функциональным изменением без предварительного обсуждения.

## Pull Request

Основная площадка для разработки Oreluno и приёма Pull Request находится в [репозитории Oreluno на GitHub](https://github.com/404-undef/oreluno). Другие размещения проекта считаются зеркалами, если владелец проекта не объявил иное.

В дальнейшем основной может стать другая площадка. Тогда расположение репозитория, порядок отправки вкладов и способ принятия CLA будут указаны в этом документе и материалах проекта. Смена площадки не изменяет публичную лицензию Oreluno или условия уже принятых CLA.

Pull Request должен:

1. кратко объяснять проблему или цель изменения
2. описывать выбранное решение
3. указывать, как изменение было проверено
4. отдельно отмечать несовместимые изменения публичного API
5. не содержать посторонних изменений, не относящихся к заявленной задаче

До объединения внешнего Pull Request потребуется принять действующую версию CLA через CLA Assistant, подключённый к репозиторию.

Без принятого CLA внешний код в `main` не объединяется.

## Коммиты

Сообщения коммитов могут быть на русском или английском языке и должны кратко описывать суть изменения.

Один коммит по возможности должен содержать одно завершённое изменение.

## Обсуждение до реализации

Перед крупным изменением сначала создайте обсуждение или Issue. Это относится к новым подсистемам, публичным интерфейсам, архитектурным перестроениям, зависимостям и изменениям лицензионной модели.

Это снижает вероятность работы над изменением, которое не будет принято.

## Безопасность и права третьих лиц

Не публикуйте в Issue, Pull Request или исходном коде:

- секреты и ключи доступа
- персональные данные без законного основания
- закрытый код третьих лиц
- материалы, для которых отсутствует право на распространение

Если вклад содержит сторонний материал, кроме уже находящегося в репозитории материала Oreluno, явно укажите в Pull Request каждую такую часть, её источник, автора (если он известен) и применимую лицензию. Не представляйте сторонний материал как собственный вклад и получите письменное одобрение владельца проекта в Pull Request до включения материала в Oreluno.

Если применимую лицензию или право Oreluno использовать и распространять материал подтвердить невозможно, не отправляйте его.

Если вы обнаружили проблему, связанную с правами на уже принятый код, свяжитесь с владельцем проекта:

**i@undef.site**

---

# Contributing to Oreluno

Thank you for your interest in Oreluno.

Please read the rules below before submitting changes. They exist both to maintain code quality and to keep the rights associated with accepted contributions clear for contributors and the project.

## Before submitting a contribution

Make sure that:

- you are the author of the code or otherwise have sufficient authority to contribute it
- the contribution does not violate obligations to an employer, client, or other organization
- third-party code or material is not copied into the project without a compatible license and the necessary permissions
- code produced with automated or generative tools has been carefully reviewed by you for correctness, security, and compliance with the project requirements
- you are willing to accept the current `CLA.md` through CLA Assistant before your Pull Request is merged

If you are contributing on behalf of a company or another organization, contact the Project Owner first. A separate entity contributor agreement may be required.

## Contribution licensing

You retain the copyright in your own Contribution.

By accepting the CLA, you grant the Oreluno Project Owner the additional rights required to use, modify, distribute, commercially license, sublicense, and relicense accepted Contributions.

If a Contribution is included in a public release of Oreluno, that Contribution also remains available under the public license applicable to Oreluno on its Submission Date.

The complete legal terms are defined in [`CLA.md`](CLA.md).

Public use of Oreluno is governed by [`LICENSE.md`](LICENSE.md). Information about commercial licensing is available in [`COMMERCIAL.md`](COMMERCIAL.md).

## Preparing changes

Before opening a Pull Request, run:

```powershell
cargo fmt
cargo check
cargo clippy
cargo test
```

All commands must complete successfully. `cargo clippy` should not introduce new warnings without a documented reason.

When a change adds behavior or fixes a defect, add a test where practical.

## Code and documentation

Oreluno prefers:

- clear naming and explicit contracts
- no unexplained numeric constants or coefficients
- brief explanations for unusual values and formulas
- comments and documentation may be written in Russian or English, with no mandatory priority between them
- public API documentation, including future `cargo doc` documentation, may be written in Russian, in English, or in both languages
- established API names and technical terms should use the form that provides the clearest and most precise meaning
- minimal dependencies, with a clear reason for every new dependency

Avoid combining a large architectural refactor with an unrelated functional change unless it has been discussed beforehand.

## Pull Requests

The [Oreluno repository on GitHub](https://github.com/404-undef/oreluno) is currently the canonical platform for Oreluno development and Pull Request submission. Other project locations are considered mirrors unless the Project Owner explicitly announces otherwise.

Any other platform may be designated as canonical in the future. If that happens, the current location of the primary repository, the contribution process, and the method for accepting the CLA will be stated in this document and the project's official materials. A platform change does not by itself alter Oreluno's public license or the terms of previously accepted CLAs.

A Pull Request should:

1. briefly explain the problem or purpose of the change
2. describe the chosen solution
3. explain how the change was tested
4. clearly identify breaking changes to the public API
5. avoid unrelated modifications

Before an external Pull Request is merged, the contributor must accept the current CLA through CLA Assistant configured for the repository.

External code is not merged into `main` without an accepted CLA.

## Commits

Commit messages may be written in Russian or English and should briefly and unambiguously describe the purpose of the change.

Where practical, one commit should represent one logically complete change.

## Discuss major changes first

For new subsystems, public interfaces, architectural restructuring, new dependencies, or licensing changes, open an Issue or discussion before implementation.

This reduces the risk of spending substantial effort on a change that cannot be accepted.

## Security and third-party rights

Do not publish in Issues, Pull Requests, or source code:

- secrets or access credentials
- personal data without a lawful basis
- confidential third-party code
- material that you do not have the right to distribute

If a contribution contains third-party material other than Oreluno material already present in the repository, clearly identify each such portion in the Pull Request and provide its source, author (if known), and applicable license. Do not represent third-party material as your own Contribution, and obtain the Project Owner's written approval in the Pull Request before the material is included in Oreluno.

If the applicable license or Oreluno's right to use and redistribute the material cannot be confirmed, do not submit it.

If you discover a rights-related issue in already accepted code, contact the Project Owner:

**i@undef.site**
