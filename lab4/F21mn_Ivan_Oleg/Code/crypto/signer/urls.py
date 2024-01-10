from django.urls import path

from . import views

urlpatterns = [
    path("", views.index, name="index"),
    path("signer/", views.signer, name="signer"),
    path("checker/", views.checker, name="checker"),
]
