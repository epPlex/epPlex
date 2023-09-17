export type Ephemerality = {
  "version": "0.0.1",
  "name": "ephemerality",
  "instructions": [
    {
      "name": "tokenCreate",
      "accounts": [
        {
          "name": "mint",
          "isMut": true,
          "isSigner": false,
          "docs": [
            "CHECK"
          ]
        },
        {
          "name": "programDelegate",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "payer",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "rent",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "token22Program",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "params",
          "type": {
            "defined": "TokenCreateParams"
          }
        }
      ]
    },
    {
      "name": "tokenBurn",
      "accounts": [
        {
          "name": "mint",
          "isMut": true,
          "isSigner": false,
          "docs": [
            "CHECK"
          ]
        },
        {
          "name": "programDelegate",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "tokenAccount",
          "isMut": true,
          "isSigner": false,
          "docs": [
            "CHECK"
          ]
        },
        {
          "name": "token22Program",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "params",
          "type": {
            "defined": "TokenBurnParams"
          }
        }
      ]
    },
    {
      "name": "programDelegateCreate",
      "accounts": [
        {
          "name": "programDelegate",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "payer",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "params",
          "type": {
            "defined": "ProgramDelegateCreateParams"
          }
        }
      ]
    },
    {
      "name": "programDelegateClose",
      "accounts": [
        {
          "name": "programDelegate",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "payer",
          "isMut": true,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "params",
          "type": {
            "defined": "ProgramDelegateCloseParams"
          }
        }
      ]
    }
  ],
  "accounts": [
    {
      "name": "programDelegate",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "bump",
            "docs": [
              "The bump, used for PDA validation."
            ],
            "type": "u8"
          }
        ]
      }
    }
  ],
  "types": [
    {
      "name": "ProgramDelegateCloseParams",
      "type": {
        "kind": "struct",
        "fields": []
      }
    },
    {
      "name": "ProgramDelegateCreateParams",
      "type": {
        "kind": "struct",
        "fields": []
      }
    },
    {
      "name": "TokenBurnParams",
      "type": {
        "kind": "struct",
        "fields": []
      }
    },
    {
      "name": "TokenCreateParams",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "destroyTimestampOffset",
            "type": "i64"
          }
        ]
      }
    }
  ],
  "errors": [
    {
      "code": 6000,
      "name": "InvalidCalculation",
      "msg": "Invalid calculation"
    }
  ]
};

export const IDL: Ephemerality = {
  "version": "0.0.1",
  "name": "ephemerality",
  "instructions": [
    {
      "name": "tokenCreate",
      "accounts": [
        {
          "name": "mint",
          "isMut": true,
          "isSigner": false,
          "docs": [
            "CHECK"
          ]
        },
        {
          "name": "programDelegate",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "payer",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "rent",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "token22Program",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "params",
          "type": {
            "defined": "TokenCreateParams"
          }
        }
      ]
    },
    {
      "name": "tokenBurn",
      "accounts": [
        {
          "name": "mint",
          "isMut": true,
          "isSigner": false,
          "docs": [
            "CHECK"
          ]
        },
        {
          "name": "programDelegate",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "tokenAccount",
          "isMut": true,
          "isSigner": false,
          "docs": [
            "CHECK"
          ]
        },
        {
          "name": "token22Program",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "params",
          "type": {
            "defined": "TokenBurnParams"
          }
        }
      ]
    },
    {
      "name": "programDelegateCreate",
      "accounts": [
        {
          "name": "programDelegate",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "payer",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "params",
          "type": {
            "defined": "ProgramDelegateCreateParams"
          }
        }
      ]
    },
    {
      "name": "programDelegateClose",
      "accounts": [
        {
          "name": "programDelegate",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "payer",
          "isMut": true,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "params",
          "type": {
            "defined": "ProgramDelegateCloseParams"
          }
        }
      ]
    }
  ],
  "accounts": [
    {
      "name": "programDelegate",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "bump",
            "docs": [
              "The bump, used for PDA validation."
            ],
            "type": "u8"
          }
        ]
      }
    }
  ],
  "types": [
    {
      "name": "ProgramDelegateCloseParams",
      "type": {
        "kind": "struct",
        "fields": []
      }
    },
    {
      "name": "ProgramDelegateCreateParams",
      "type": {
        "kind": "struct",
        "fields": []
      }
    },
    {
      "name": "TokenBurnParams",
      "type": {
        "kind": "struct",
        "fields": []
      }
    },
    {
      "name": "TokenCreateParams",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "destroyTimestampOffset",
            "type": "i64"
          }
        ]
      }
    }
  ],
  "errors": [
    {
      "code": 6000,
      "name": "InvalidCalculation",
      "msg": "Invalid calculation"
    }
  ]
};
