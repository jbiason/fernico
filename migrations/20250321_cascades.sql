PRAGMA foreign_keys = OFF;

CREATE TABLE entry_new (
	id INTEGER PRIMARY KEY NOT NULL,
	day INTEGER NOT NULL,
	month INTEGER NOT NULL,
	year INTEGER NOT NULL,
	account CHAR(10) NOT NULL,
	credit DECIMAL(15,2),
	debit DECIMAL(15,2),
	CONSTRAINT fk_account
		FOREIGN KEY (account) 
		REFERENCES account(id)
		ON DELETE CASCADE
);
INSERT INTO entry_new SELECT * from entry;
DROP TABLE entry;
ALTER TABLE entry_new RENAME TO entry;

CREATE TABLE acc_balance_new (
	account CHAR(10) NOT NULL,
	month INTEGER NOT NULL,
	year INTEGER NOT NULL,
	previous_credit DECIMAL(16,2) NOT NULL,
	previous_debit DECIMAL(16,2) NOT NULL,
	credit DECIMAL(16, 2) NOT NULL,
	debit DECIMAL(16, 2) NOT NULL,
	PRIMARY KEY (account, month, year)
	CONSTRAINT fk_acc
		FOREIGN KEY (account)
		REFERENCES account(id)
		ON DELETE CASCADE
);
INSERT INTO acc_balance_new SELECT * from acc_balance;
DROP TABLE acc_balance;
ALTER TABLE acc_balance_new RENAME TO acc_balance;

PRAGMA foreign_keys = ON;
