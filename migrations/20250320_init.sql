CREATE TABLE IF NOT EXISTS account (
	id CHAR(10) PRIMARY KEY NOT NULL,
	name TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS entry (
	id INTEGER PRIMARY KEY NOT NULL,
	calendar CHAR(10) NOT NULL,		-- event date ("calendar" to avoid mixing with date() function)
	account CHAR(10) NOT NULL,
	credit DECIMAL(15,2),
	debit DECIMAL(15,2),
	note TEXT,
	CONSTRAINT fk_account
		FOREIGN KEY (account)
		REFERENCES account(id)
		ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS balance (
	calendar CHAR(10) NOT NULL,
	previous_credit DECIMAL(16, 2) NOT NULL,
	previous_debit DECIMAL(16, 2) NOT NULL,
	credit DECIMAL(16, 2) NOT NULL,
	debit DECIMAL(16, 2) NOT NULL,
	PRIMARY KEY (calendar)
);

CREATE TABLE IF NOT EXISTS acc_balance (
	account CHAR(10) NOT NULL,
	calendar CHAR(10) NOT NULL,
	previous_credit DECIMAL(16,2) NOT NULL,
	previous_debit DECIMAL(16,2) NOT NULL,
	credit DECIMAL(16, 2) NOT NULL,
	debit DECIMAL(16, 2) NOT NULL,
	CONSTRAINT pk_acc_balance
		PRIMARY KEY (account, calendar),
	CONSTRAINT fk_account
		FOREIGN KEY (account)
		REFERENCES account(id)
		ON DELETE CASCADE
);
