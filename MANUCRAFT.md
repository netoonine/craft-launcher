# MANUCRAFT (fork do ArtCraft Launcher)

Este repositorio e `netoonine/craft-launcher`, branch `manucraft`: o launcher **MANUCRAFT**, que
instala e atualiza os forks dos 13 Craft apps a partir dos GitHub Releases de
`github.com/netoonine/<id>` e se atualiza a partir de `netoonine/craft-launcher`. E um fork de
`storytold/craft-launcher` (remote `upstream`), sincronizado com frequencia.

> MANUCRAFT is based on ArtCraft Launcher by the ArtCraft team; not affiliated with or endorsed by them.

Regra de ouro (ver `../CLAUDE.md`): codigo novo em arquivo novo; nos arquivos do upstream so
ganchos de 1-2 linhas marcados com `MANUCRAFT` (`grep -rn MANUCRAFT`); registrar tudo aqui.

## 1. O que muda em relacao ao upstream

| Tema | Upstream | MANUCRAFT |
|---|---|---|
| Origem dos apps | `storytold/<id>` | `netoonine/<id>` (catalogo, `GITHUB_OWNER`) |
| Origem do proprio launcher | `storytold/craft-launcher` | `netoonine/craft-launcher` |
| Feed de releases | `https://api.storyteller.ai/v1/craft_apps/release_info` primeiro, GitHub como reserva | **desligado**: nenhum pedido aos servidores da storyteller; so GitHub (API, ou o redirect `releases/latest` + `SHA256SUMS.txt` quando a cota acaba) |
| Assinatura no macOS | exige Developer ID do time `DJ6XS33FX8` | aceita **qualquer assinatura integra** (ad-hoc inclusive): `codesign --verify --deep --strict`. O SHA-256 do download continua obrigatorio |
| Versao | semver puro | `v<upstream>+m<N>` (build MANUCRAFT, secao 3) |
| Nome | ArtCraft Launcher | **MANUCRAFT** (janela, menu bar, Ajustes, notificacoes, web, DMG, Info.plist) |
| Bundle id / app id | `ai.storyteller.artcraft-launcher` | `ai.manucraft.launcher` (Info.plist, LaunchAgent, notificacoes, pasta de ajustes do eframe) |
| Pastas de dados | `ArtCraft Launcher` | `MANUCRAFT` (`~/Library/Application Support/MANUCRAFT`, `~/Library/Caches/MANUCRAFT`); ajustes em `~/Library/Application Support/ai.manucraft.launcher/app.ron` |
| Instancia unica | `127.0.0.1:47413`, prefixo `artcraft-launcher:` | `127.0.0.1:47431`, prefixo `manucraft:` (roda lado a lado com o oficial) |
| User-Agent | `ArtCraftLauncher/<v>` | `MANUCRAFT/<v>` |
| Links | Discord, getartcraft.com, redes sociais | nenhum; o unico link "GitHub" abre `https://github.com/netoonine/manucraft`. O link "Website" de cada app abre o fork no GitHub |
| Marca | logo/"A" da ArtCraft (marca registrada, nao open source) | removidos (`docs/brand/`, `docs/screenshots/`); marca e wordmark proprios em `assets/manucraft/` |
| Apps detectados | bundle id `ai.storyteller.<id>` | **igual** (os forks mantem `ai.storyteller.<id>`) |

O que **nao** mudou, de proposito (para o diff ficar pequeno):

- Nomes de crates, binario, stem dos assets e nomes de arquivo continuam `artcraft-launcher*`
  (`cargo run -p artcraft-launcher`, `artcraft-launcher-<v>-macos-aarch64.dmg`,
  `assets/app-icon/artcraft-launcher.*`, `hicolor/.../ai.storyteller.artcraft-launcher.png`).
  `catalog::LAUNCHER.id` continua `artcraft-launcher`; so o nome exibido e o bundle id mudaram.
- Variaveis de ambiente continuam `ARTCRAFT_LAUNCHER_*` (ex.: `ARTCRAFT_LAUNCHER_GITHUB_TOKEN`
  para subir a cota do GitHub de 60 para 5000 pedidos/hora; `ARTCRAFT_LAUNCHER_LOCALE`).
- Fotos da pagina de detalhes: continuam vindo (so leitura, JPEG, com cache) de
  `raw.githubusercontent.com/storytold/craft-launcher-assets` (`core/src/details.rs:53`).
  E o unico acesso a algo da storytold em tempo de execucao.
- Empacotamento de Windows, Linux, FreeBSD e Flatpak: compila, mas nao foi adaptado (ids
  `ai.storyteller.artcraft-launcher` nos `.desktop`/Flatpak, chave de registro
  `Learning Machines LLC\ArtCraft Launcher` do MSI, workflows em `.github/`). Os workflows do
  upstream nao rodam no fork (runners pagos, segredos da Apple).
- O modulo `feed.rs` continua existindo (o espelho local `cargo xtask local` usa o mesmo formato).

## 2. Arquivos

### Nossos (nunca conflitam)

| Arquivo | O que faz |
|---|---|
| `MANUCRAFT.md` | este mapa |
| `crates/core/src/manucraft.rs` | `GITHUB_OWNER`, `PROJECT_URL`, `LAUNCHER_REPO`, `LAUNCHER_NAME`, `LAUNCHER_BUNDLE_ID`, `ABOUT`; `build_number`/`build_order` (ordem dos builds `+m<N>`) e testes |
| `crates/native/src/manucraft.rs` | `NO_FEED`; `with_build` (o `.app` so diz `0.9.0`, o registro do launcher guarda `0.9.0+m2`); `signature_intact` (codesign --verify --deep --strict); testes, incl. `installs_a_real_dmg` (ignorado, roda com um DMG real) |
| `crates/ui-egui/src/manucraft.rs` | `rebrand` (troca "ArtCraft Launcher"/"ArtCraft" por MANUCRAFT nas traducoes de todos os idiomas); `catalog_source` (catalogo do ingles gerado so com as frases que citam o produto); `community_links()` = false; testes |
| `assets/manucraft/` | marca (`manucraft-mark.svg`, `manucraft-mark-black.svg`), wordmark (`manucraft-wordmark*.svg`, Archivo contornada), PNGs gerados (bandeja 44/64 px, wordmark para o instalador Windows) e fontes `src/` (wordmark e fundo do DMG com `<text>`) |
| `packaging/manucraft-icons.sh` | gera icone do app (`.icns`, `.ico`, `hicolor`, 1024) e da bandeja a partir da marca |

### Do upstream, substituidos por inteiro (em conflito: ficar com o nosso)

`README.md`, `packaging/macos/Info.plist.in`, `assets/app-icon/README.md`,
`assets/app-icon/LICENSE.txt`, os binarios/SVG gerados em `assets/app-icon/`,
`packaging/macos/dmg/background.svg|.tiff` e `dmg-layout.DS_Store` (gerados; refazer com os
comandos da secao 5). Apagados: `docs/brand/`, `docs/screenshots/` (marca da ArtCraft; se o
upstream mexer neles, manter apagados).

### Do upstream, com ganchos `MANUCRAFT` (linhas aproximadas)

| Arquivo | Linhas | Gancho |
|---|---|---|
| `Cargo.toml` | 6-7, 11 | `version = "<upstream>+m<N>"` (comentario na linha de cima); `repository` |
| `Cargo.lock` | - | segue a versao (`0.4.1+m1`) dos crates do workspace; em conflito, aceitar o upstream e rodar `cargo build` |
| `crates/core/src/lib.rs` | 13 | `pub mod manucraft;` |
| `crates/core/src/catalog.rs` | 56, 71, 108-208, 213-238, 255-268 | `bundle_id()` do launcher = `ai.manucraft.launcher`; `website()` = repo do fork; 13 repos `netoonine/<id>`; `WEBSITE_URL`/`GITHUB_URL`/`DISCORD_URL` = projeto; `SOCIAL` vazio; `GITHUB_OWNER`; `LAUNCHER` (nome, repo); `LAUNCHER_REPO`; testes |
| `crates/core/src/version.rs` | 107 | `compare` desempata por `build_order` (`+m<N>`) |
| `crates/core/src/feed.rs` | 26, 436 | `FEED_URL = ""`; fixtures de teste com `netoonine` |
| `crates/native/src/lib.rs` | 18 | `pub mod manucraft;` |
| `crates/native/src/backend.rs` | 629, 1116 | `feed_and_fallback` retorna sem pedir o feed (GitHub a cada 5 min ate tudo carregar, depois de hora em hora por app; Atualizar pede na hora); teste `manucraft_never_asks_the_feed` |
| `crates/native/src/install.rs` | 70, 987 | instalacao por DMG grava registro no manifesto (com o `+m<N>`); verificacao de assinatura via `signature_intact` |
| `crates/native/src/detect.rs` | 63 | `from_manifest` aplica `with_build` |
| `crates/native/src/single_instance.rs` | 26-32 | porta 47431, prefixos `manucraft:` (e testes) |
| `crates/native/src/paths.rs` | 14, 93 | pastas `MANUCRAFT` / `manucraft` / `ManucraftData` |
| `crates/native/src/http.rs` | 12 | User-Agent `MANUCRAFT/<v>` |
| `crates/native/src/login.rs` | 12, 38, 73, 75, 89-91 | LaunchAgent `ai.manucraft.launcher.plist`; nomes de inicializacao |
| `crates/native/src/self_update.rs` | 336-378 | teste de troca do bundle usa `MANUCRAFT.app` / `ai.manucraft.launcher` |
| `crates/ui-egui/src/lib.rs` | 26 | `mod manucraft;` |
| `crates/ui-egui/src/i18n/lang.rs` | 223 | catalogos passam por `manucraft::catalog_source` |
| `crates/ui-egui/src/i18n/translate.rs` | 67, 79 | `trn` e `message` passam por `rebrand` |
| `crates/ui-egui/src/icons.rs` | 25-52 | wordmark e marca MANUCRAFT, proporcao 1081/102 |
| `crates/ui-egui/src/views.rs` | 83, 95, 321-322 | wordmark no cabecalho; sem link Discord; rodape so com GitHub |
| `crates/ui-egui/src/dialogs.rs` | 194, 345, 352, 356 | subtitulo "MANUCRAFT <versao>"; aviso `ABOUT` no lugar do texto da comunidade; botoes Discord/Website escondidos |
| `crates/ui-egui/src/i18n/tests.rs` | 194 | "ArtCraft Launcher" aparece como "MANUCRAFT" em todos os idiomas |
| `crates/ui-egui/tests/ui.rs`, `tests/hungarian.rs` | ver `grep` | testes de links/Discord/rodape/nome adaptados (mesma intencao) |
| `apps/artcraft-launcher/src/main.rs` | 31, 61, 107, 136, 408 | `APP_ID`, titulo da janela, notificacoes, log |
| `apps/artcraft-launcher/src/tray.rs` | 30-35, 52, 67, 76, 134 | icone "M" da bandeja; menu sem site/Discord; tooltip |
| `apps/artcraft-launcher/build.rs` | 16-17 | ProductName (Windows) |
| `apps/artcraft-launcher-web/index.html` | 6, 20 | titulo / "Loading MANUCRAFT" |
| `packaging/macos/package.sh` | 4, 21, 43, 46 | `APP_NAME="MANUCRAFT"`; `SHORT_VERSION` sem `+m<N>`; DMG com `+` trocado por `-` |
| `packaging/macos/verify.sh` | 20, 37 | idem (verify.sh exige Developer ID/notarizacao, entao falha em build ad-hoc; use os testes da secao 5) |
| `packaging/macos/dmg/generate.py` | 27, 32 | volume `MANUCRAFT`, `MANUCRAFT.app` |
| `packaging/macos/dmg/README.md` | 10, 12, 17, 32 | descricao do fundo novo e como regenerar |
| `packaging/icons.sh` | 15 | `exec packaging/manucraft-icons.sh` |
| `packaging/windows/package.ps1` | 116 | wordmark MANUCRAFT na arte do instalador |
| `xtask/src/version.rs` | 43, 146 | `validate` aceita `+m<N>` |
| `xtask/src/local.rs` | 189, 457 | espelho local usa `netoonine/<id>` |
| `AGENTS.md` | 3-10, 70, 81 | aviso do fork; downloads e regra de apps de terceiros passam a ser `netoonine/` |
| `ATTRIBUTION.md`, `NOTICE` | ver `grep` | ativos MANUCRAFT; marca ArtCraft removida; frase de "based on" |

## 3. Releases e versionamento

### Regra

Toda release MANUCRAFT (de app ou do launcher) tem a tag

```
v<UPSTREAM>+m<N>
```

- `UPSTREAM` = a versao do upstream que o build carrega, `X.Y.Z`, exatamente o que o app grava no
  proprio Info.plist (a `[workspace.package] version` do `Cargo.toml` do app; nao mude isso).
- `N` = numero do build MANUCRAFT daquela versao: 1, 2, 3... Recomeca em 1 quando `UPSTREAM` muda.
  Exemplos: `v0.9.0+m1`, `v0.9.0+m2`, `v0.9.1+m1`.
- Ordem que o launcher usa: `0.9.0` < `0.9.0+m1` < `0.9.0+m2` < `0.9.0+m10` < `0.9.1+m1`.
  (Semver ignora o `+build` ao ordenar; o MANUCRAFT desempata por `N` numericamente.)
- **Nunca** use hifen na tag (`v0.9.0-m2`): hifen e pre-release em semver, a release ficaria
  escondida (e abaixo de `0.9.0`). `+m<N>` nunca e pre-release.
- **Nunca** marque a release como pre-release nem deixe como rascunho; deixe-a como "Latest"
  (padrao do `gh release create`).

### Release de um app (`netoonine/<id>`)

1. Build no fork do app (ver `../CLAUDE.md`):
   `cd <id> && CRAFT_FONTS_DIR=../craft-fonts packaging/macos/package.sh --arch aarch64`
   -> `dist/release/<id>-<UPSTREAM>-macos-aarch64.dmg`.
2. Assets da release, com estes nomes:
   - o DMG **com o nome que o package.sh do app gera**: `<id>-<UPSTREAM>-macos-aarch64.dmg`
     (tambem servem `-macos-universal`, `-macos-arm64`, `-macos-x86_64`). Sem `+` no nome do
     arquivo (o GitHub pode renomear); o `N` fica so na tag.
   - `SHA256SUMS.txt`, obrigatorio, no formato do `shasum -a 256`
     (`<64 hex>  <nome do arquivo>`). Sem ele (e sem o digest do GitHub) o launcher nao instala;
     e ele que permite achar a release sem gastar cota da API.
3. Conteudo do DMG: `<Nome>.app` na raiz, com o nome exato do catalogo: `PhotoCraft.app`,
   `VectorCraft.app`, `FilmCraft.app`, `LightCraft.app`, `PdfCraft.app`, `EffectCraft.app`,
   `DesignCraft.app`, `WordCraft.app`, `DeckCraft.app`, `GridCraft.app`, `CADCraft.app`,
   `SoundCraft.app`, `FigCraft.app`. No Info.plist: `CFBundleIdentifier` = `ai.storyteller.<id>`;
   `<Nome>Version` (ou `CFBundleShortVersionString`) = `UPSTREAM`. Assinado (ad-hoc serve) e sem
   alteracao depois de assinar: `codesign --verify --deep --strict <Nome>.app` tem de passar.
4. Publicar:

   ```sh
   V=0.9.0; N=2; ID=photocraft
   cd dist/release && shasum -a 256 $ID-$V-macos-aarch64.dmg > SHA256SUMS.txt
   gh release create "v$V+m$N" --repo netoonine/$ID --title "PhotoCraft $V+m$N" \
     --notes "..." $ID-$V-macos-aarch64.dmg SHA256SUMS.txt
   ```

O que o launcher faz: mostra "0.9.0+m2" como ultima versao; instala; confere SHA-256, bundle id,
versao (`0.9.0` do Info.plist bate com `0.9.0+m2`: os numeros sao iguais) e assinatura; grava
`0.9.0+m2` em `~/Library/Application Support/MANUCRAFT/installed.json`. Na deteccao, o `.app`
diz `0.9.0` e o registro completa para `0.9.0+m2` -> "atualizado". Quando sair `v0.9.0+m3`,
mostra "atualizacao disponivel"; sem loop. Um app instalado a mao (sem registro) aparece como
`0.9.0` e recebe a oferta de atualizar para `0.9.0+m2` uma vez; depois de instalado pelo
launcher fica registrado. Se o `.app` for trocado por outra versao (ex.: `0.9.1`), vale o que
esta no disco.

### Release do launcher (`netoonine/craft-launcher`)

1. Em `Cargo.toml`, `[workspace.package] version = "<UPSTREAM>+m<N>"` (hoje `0.4.1+m1`;
   `UPSTREAM` = a versao do upstream mesclada). Ex.: `cargo xtask version set 0.4.1+m2`.
2. `packaging/macos/package.sh --arch aarch64` -> `target/macos-package/MANUCRAFT.app` e
   `dist/release/artcraft-launcher-<UPSTREAM>-m<N>-macos-aarch64.dmg` (o `+` vira `-` no nome do
   arquivo; o stem `artcraft-launcher` e o que a auto-atualizacao procura). No Info.plist:
   `CFBundleShortVersionString` = `UPSTREAM`, `MANUCRAFTVersion` = `UPSTREAM+m<N>`.
3. `shasum -a 256 artcraft-launcher-*-macos-aarch64.dmg > SHA256SUMS.txt` e
   `gh release create "v<UPSTREAM>+m<N>" --repo netoonine/craft-launcher --title "MANUCRAFT <UPSTREAM>+m<N>" <dmg> SHA256SUMS.txt`.

A auto-atualizacao compara a versao compilada (`CARGO_PKG_VERSION`, com o `+m<N>`) com a tag;
instala por cima do `MANUCRAFT.app` onde ele estiver (bundle id `ai.manucraft.launcher`,
assinatura integra, versao igual a da tag) e reabre.

## 4. Sincronizar com o upstream

```sh
git fetch upstream && git merge upstream/main     # na branch manucraft
grep -rn "storytold/" crates apps xtask           # nada novo fora de testes e details.rs
```

- Conflitos em linhas com `MANUCRAFT`: manter o gancho e aplicar a mudanca do upstream em volta.
- `Cargo.toml` version: usar a nova versao do upstream + `+m1`.
- Arquivos "substituidos por inteiro" (secao 2): ficar com o nosso e trazer so o que for util.
- App novo no catalogo: o teste `suite_has_seven_classic_and_six_new_apps_with_unique_ids` falha
  enquanto o repo nao for `netoonine/<id>`; trocar e adicionar `// MANUCRAFT`.
- Frase nova com "ArtCraft Launcher" na interface: o rebrand cobre sozinho (todas as linguas);
  links novos para getartcraft/Discord precisam de gancho com `community_links()`.
- Depois: secao 5.

## 5. Como testar

```sh
export PATH="/opt/homebrew/opt/rustup/bin:$PATH"
cargo test --workspace
cargo clippy --workspace --all-targets -- -D warnings
cargo fmt --all -- --check
cargo xtask wasm
packaging/macos/package.sh --arch aarch64
codesign --verify --deep --strict target/macos-package/MANUCRAFT.app
# o DMG passa pelas mesmas checagens que o launcher faz ao instalar:
MANUCRAFT_DMG=$PWD/dist/release/artcraft-launcher-0.4.1-m1-macos-aarch64.dmg MANUCRAFT_APP=artcraft-launcher \
  MANUCRAFT_VERSION=0.4.1+m1 cargo test -p artcraft-launcher-native installs_a_real_dmg -- --include-ignored
# idem para um app: MANUCRAFT_DMG=<dmg do app> MANUCRAFT_APP=photocraft MANUCRAFT_VERSION=0.9.0+m2
```

Icones e fundo do DMG (precisa de `cargo install resvg usvg --locked --version 0.48.1` e, para o
`.DS_Store`, `pip install 'pillow>=12' ds_store==1.3.3 mac_alias==2.2.3`):

```sh
packaging/icons.sh
usvg --skip-system-fonts --use-font-file assets/fonts/Archivo-Variable.ttf \
  --use-font-file assets/fonts/GeistMono-Variable.ttf \
  assets/manucraft/src/dmg-background.src.svg packaging/macos/dmg/background.svg
python3 packaging/macos/dmg/generate.py
```

Capturas da interface: `ARTCRAFT_LAUNCHER_SNAPSHOTS=<pasta> cargo test -p artcraft-launcher-ui-egui --test ui`.

## 6. Pendencias

- Logo: a marca "M" e o wordmark sao provisorios (simples, originais); trocar
  `assets/manucraft/manucraft-mark.svg` e os wordmarks e rodar a secao 5.
- Os forks ainda nao tem releases: o launcher mostra "sem release" ate a primeira `v<X.Y.Z>+m1`.
- `https://github.com/netoonine/manucraft` (link "GitHub") e privado: so abre para quem tem acesso.
- Windows/Linux/FreeBSD/Flatpak e workflows do GitHub nao foram adaptados (secao 1).
