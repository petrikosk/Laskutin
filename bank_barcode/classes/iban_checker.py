import string


class IbanChecker:
    def __init__(self, iban):
        self.LETTERS = {ord(d): str(i) for i, d in enumerate(string.digits + string.ascii_uppercase)}
        self.iban = iban.replace(" ", "")  # Remove whitespaces

    def _number_iban(self, iban):
        return (iban[4:] + iban[:4]).translate(self.LETTERS)

    def generate_iban_check_digits(self):
        number_iban = self._number_iban(self.iban[:2] + '00' + self.iban[4:])
        return '{:0>2}'.format(98 - (int(number_iban) % 97))

    def valid_iban(self):
        return int(self._number_iban(self.iban)) % 97 == 1

    def check(self):
        try:
            if self.generate_iban_check_digits() == self.iban[2:4] and self.valid_iban():
                return True
        except ValueError:
            return False
        return False
