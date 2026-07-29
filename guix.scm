; SPDX-License-Identifier: MPL-2.0
;; guix.scm — GNU Guix package definition for universal-language-server-plugin
;; Usage: guix shell -f guix.scm

(use-modules (guix packages)
             (guix build-system gnu)
             (guix licenses))

(package
  (name "universal-language-server-plugin")
  (version "0.1.0")
  (source #f)
  (build-system gnu-build-system)
  (synopsis "universal-language-server-plugin")
  (description "universal-language-server-plugin — part of the hyperpolymath ecosystem.")
  (home-page "https://github.com/hyperpolymath/universal-language-server-plugin")
  (license mpl2.0))
