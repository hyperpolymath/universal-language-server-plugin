# Maintainers

This document lists the maintainers of the Universal Language Connector project.

## Project Leadership

### Creator & Lead Maintainer
- **Role**: Project creator, architecture decisions, final approvals
- **Scope**: All components (server, clients, documentation)
- **TPCF Perimeter**: Perimeter 3 (Community Sandbox - Open)

## Component Maintainers

### Rust Server
- **Maintainer**: [To be assigned]
- **Components**: LSP handler, HTTP API, WebSocket server, conversion core
- **Responsibilities**: Code review, security patches, performance optimization

### Editor Clients
- **VS Code**: [To be assigned]
- **Neovim**: [To be assigned]
- **Emacs**: [To be assigned]
- **JetBrains**: [To be assigned]
- **Sublime Text**: [To be assigned]
- **Zed/Helix**: [To be assigned]

### Web UI
- **Maintainer**: [To be assigned]
- **Components**: HTML/CSS/JS dashboard, real-time updates

### Documentation
- **Maintainer**: [To be assigned]
- **Components**: README, API docs, guides, examples

### Infrastructure
- **Maintainer**: [To be assigned]
- **Components**: CI/CD, Docker, deployment, build system

## Maintainer Responsibilities

### Code Review
- Review pull requests within 48 hours
- Ensure code quality and test coverage
- Verify LSP compliance
- Check security implications

### Community Management
- Respond to issues within 72 hours
- Welcome new contributors
- Enforce Code of Conduct
- Maintain positive community culture

### Technical Decisions
- Architecture decisions require consensus
- Breaking changes need RFC process
- Performance targets must be maintained
- Security fixes prioritized

### Release Management
- Follow semantic versioning
- Maintain CHANGELOG.md
- Create release notes
- Tag releases properly

## Becoming a Maintainer

### Criteria
1. **Sustained Contribution**: 3+ months of quality contributions
2. **Technical Expertise**: Deep understanding of component area
3. **Community Trust**: Positive interactions, helpful reviews
4. **Time Commitment**: Available for regular code review and issue triage

### Process
1. Existing maintainer nominates candidate
2. Candidate accepts nomination
3. One-week community feedback period
4. Consensus decision by current maintainers
5. Onboarding: repository access, documentation, tools

## Stepping Down

Maintainers can step down at any time by:
1. Notifying other maintainers
2. Ensuring knowledge transfer
3. Removing themselves from MAINTAINERS.md
4. Transitioning in-progress work

We deeply appreciate all maintainer contributions, past and present.

## Emeritus Maintainers

Maintainers who have stepped down with honor:

- [None yet]

## Contact

- **Mailing List**: maintainers@universal-connector.org
- **Private Channel**: [Discord/Slack/Matrix]
- **Security Contact**: security@universal-connector.org

## Governance

### Decision Making

- **Consensus**: Preferred method for all decisions
- **Lazy Consensus**: 72-hour timeout for minor changes
- **Voting**: Majority vote if consensus fails (requires 2/3 quorum)
- **Tie-breaking**: Lead maintainer has final say

### Conflict Resolution

1. **Discussion**: Open discussion in maintainer channel
2. **Mediation**: Neutral third-party mediator
3. **Voting**: If mediation fails, use voting process
4. **Escalation**: Code of Conduct committee for conduct issues

## Tri-Perimeter Contribution Framework (TPCF)

### Current Perimeter: Perimeter 3 (Community Sandbox)

**Access Level**: Open contribution
- Anyone can submit pull requests
- Maintainer review required for merge
- Two-factor authentication recommended
- Signed commits encouraged

### Future Perimeters

**Perimeter 2 (Trusted Contributors)**: [Not yet implemented]
- Regular contributors with proven track record
- Faster review process
- Limited merge access to non-critical components

**Perimeter 1 (Core Team)**: [Not yet implemented]
- Long-term maintainers
- Full repository access
- Security-sensitive component access
- Release authority

## Updates

This document should be updated when:
- New maintainers are added
- Maintainers step down
- Component ownership changes
- Governance processes evolve

---

**Last Updated**: 2025-11-22
**Version**: 1.0
