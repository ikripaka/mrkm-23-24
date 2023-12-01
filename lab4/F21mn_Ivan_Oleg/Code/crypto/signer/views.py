from django.shortcuts import render
from django.http import HttpResponse
from django.template import loader
from django.views.decorators.csrf import csrf_protect
from django.template import RequestContext
from .forms import *
from hashlib import sha256
import os
import sys
import chilkat

# Create your views here.
def sign_file(pfx,file,pfx_pass):
    crypt=chilkat.CkCrypt2()
    signed=file+'.p7s'
    init_h=open(file,"rb")
    final_h=open(signed,"wb")
    not_signed=init_h.read()
    certStore = chilkat.CkCertStore()
    success = certStore.LoadPfxFile(pfx,pfx_pass)
    if (success != True):
        return "error","Wrong password to pfx",certStore.lastErrorText()
    
    cert = certStore.GetCertificate(0)
    if (certStore.get_LastMethodSuccess() == False):
        return "error","Failed to get info from container, however pfx password was correct",certStore.lastErrorText()
    crypt.SetSigningCert(cert)
    crypt.put_HashAlgorithm("sha256")
    crypt.CreateP7S(file,signed)
    final_h.close()
    init_h.close()
    return signed,"",""

def check_file(signed,file):
    crypt=chilkat.CkCrypt2()
    result=crypt.VerifyP7S(file,signed)
    return result, crypt.lastErrorText()

def handle_uploaded_file(f):   
    with open('files/'+f.name, 'wb+') as destination:   
        for chunk in f.chunks(): 
            destination.write(chunk)   

def index(request):
    
    return render(request,'index.html',{'sign_form':sign_form(),'check_form':check_form()})


def signer(request):
    
    if request.method=="POST":
        form=sign_form(request.POST,request.FILES)
        if form.is_valid():
            handle_uploaded_file(request.FILES["pfx_file"])
            handle_uploaded_file(request.FILES["user_file"])
            pfx_file='files/'+request.FILES["pfx_file"].name
            user_file='files/'+request.FILES["user_file"].name
            pfx_pass=request.POST["password"]
            dst_file,error,error_log=sign_file(pfx_file,user_file,pfx_pass)
            if dst_file=="error":
                return render(request,'checker.html',{'check_result':"Signing failed",'bgcolor':'red','error':error,'error_log':error_log})
            with open(dst_file, 'rb') as fh:
                response = HttpResponse(fh.read(), content_type="application/octet-stream")
                response['Content-Disposition'] = 'inline; filename=' + os.path.basename(dst_file)
                fh.close()
                os.remove(dst_file)
                os.remove(pfx_file)
                os.remove(user_file)
                return response
        return HttpResponse("Signer Error: form validation failed")
    return HttpResponse("Sign file")

def checker(request):
    if request.method=="POST":
        form=check_form(request.POST,request.FILES)

        if form.is_valid():
            handle_uploaded_file(request.FILES["p7s_file"])
            handle_uploaded_file(request.FILES["user_file"])
            p7s_file='files/'+request.FILES["p7s_file"].name
            user_file='files/'+request.FILES["user_file"].name
            check_result,error=check_file(p7s_file,user_file)
            if check_result==True:
                return render(request,'checker.html',{'check_result':"Signature is valid",'bgcolor':'green'})
            else:
                return render(request,'checker.html',{'check_result':"Check failed",'bgcolor':'red','error':error})
            
        return HttpResponse("Check Error: form validation failed")
    return HttpResponse("Check sign")
