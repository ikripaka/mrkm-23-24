from django import forms

class sign_form(forms.Form):
    pfx_file = forms.FileField()
    password = forms.CharField(widget=forms.PasswordInput())
    user_file = forms.FileField()


class check_form(forms.Form):
    user_file = forms.FileField()
    p7s_file = forms.FileField()
