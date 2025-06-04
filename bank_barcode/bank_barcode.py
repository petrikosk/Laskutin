from datetime import datetime
from babel.numbers import format_decimal, parse_decimal
from invoicing.invoicer.bank_barcode.classes.iban_checker import IbanChecker
from invoicing.invoicer.referencecalculator import reference_ok


def normalise_amount(amt):
    digits = f"{format_decimal(amt, format='#,##000000.00;-#', locale='en_US')}"
    digits, decimal = digits.split('.')
    return f"{digits}{decimal}"


class BankBarcode:
    def __init__(self, iban, reference, amount, due_date):
        """
        :param str iban: Bank IBAN code
        :param str reference: Finnish invoice reference number
        :param str amount: Total amount of invoice
        :param datetime due_date: Invoice due date
        """
        amt = parse_decimal(amount, locale='fi_FI')
        if amt:
            if amt < 0:
                raise (ValueError, "Amount is negative")
            if amt > 999999.99:
                raise (ValueError, "amount is too large, max 999999,99")

        self.amount = normalise_amount(amt)

        iban_checker = IbanChecker(iban)
        if not iban_checker.check():
            raise (ValueError, "Invalid IBAN")
        # Remove two letters from the beginning
        self.iban = iban_checker.iban[2:]
        self.reference = reference.replace(" ", "")  # Remove whitespaces
        if not reference_ok(self.reference):
            raise (ValueError, "Invalid reference number")

        self.reference = self.reference.zfill(20)
        self.due_date = due_date.strftime("%y%m%d")

    def generate(self):
        return f"4{self.iban}{self.amount}000{self.reference}{self.due_date}"
