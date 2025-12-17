;;; STATE.scm — universal-language-server-plugin
;; SPDX-License-Identifier: AGPL-3.0-or-later
;; SPDX-FileCopyrightText: 2025 Jonathan D.A. Jewell

(define metadata
  '((version . "0.1.0") (updated . "2025-12-17") (project . "universal-language-server-plugin")))

(define current-position
  '((phase . "v0.1 - Security Hardening")
    (overall-completion . 35)
    (components
     ((rsr-compliance ((status . "complete") (completion . 100)))
      (containerfile ((status . "fixed") (completion . 100)))
      (flake-nix ((status . "fixed") (completion . 100)))
      (ci-security ((status . "in-progress") (completion . 70)))
      (client-migration ((status . "pending") (completion . 0)))))))

(define blockers-and-issues
  '((critical ())
    (high-priority
     (("SHA-pin CI workflow actions" . "security")
      ("Convert VS Code client to ReScript" . "rsr-policy")
      ("Convert Sublime client to ReScript" . "rsr-policy")))))

(define critical-next-actions
  '((immediate
     (("SHA-pin GitHub Actions" . high)
      ("Add security.txt" . medium)))
    (this-week
     (("Client ReScript migration" . high)
      ("Expand test coverage" . medium)
      ("Add cargo-audit to CI" . medium)))))

(define session-history
  '((snapshots
     ((date . "2025-12-15") (session . "initial") (notes . "SCM files added"))
     ((date . "2025-12-17") (session . "security-review")
      (notes . "Fixed Containerfile (mixed pkg mgr, wrong base), flake.nix (license, docker refs)")))))

(define state-summary
  '((project . "universal-language-server-plugin")
    (completion . 35)
    (blockers . 0)
    (high-priority-issues . 3)
    (updated . "2025-12-17")))
