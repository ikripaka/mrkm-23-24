import * as React from 'react';
import {useState} from 'react';
import {styled} from '@mui/material/styles';
import Button from '@mui/material/Button';
import CloudUploadIcon from '@mui/icons-material/CloudUpload';
import {Alert, Box} from "@mui/material";

const VisuallyHiddenInput = styled('input')({
    clip: 'rect(0 0 0 0)',
    clipPath: 'inset(50%)',
    height: 1,
    overflow: 'hidden',
    position: 'absolute',
    bottom: 0,
    left: 0,
    whiteSpace: 'nowrap',
    width: 1,
});

const VerifyFrom = () => {
    const [content, setContent] = useState(null);
    const [publicKey, setPublicKey] = useState(null);
    const [signature, setSignature] = useState(null);

    const [success, setSuccess] = useState("");
    const [error, setError] = useState("")


    const uploadFile = (setter) => (e) => {
        setter(e.target.files[0])
    }

    const verify = () => {
        const formData = new FormData();

        formData.append("content", content);
        formData.append("public_key", publicKey);
        formData.append("signature", signature);

        fetch("/ds/verify", { method: 'POST', body: formData })
            .then(r => r.json())
            .then(b => {
                if (b.status == 200) {
                    setError(null)
                    setSuccess(b.data)
                } else {
                    setError(b.data)
                    setSuccess(null)
                }

            })
            .catch(err => {
                setError(err)
                setSuccess(null)
            })
    }

    return (
        <Box>
            <h1>Verifying</h1>

           <Box sx={{mb: "2rem"}}>
               <Button component="label" variant="contained" startIcon={<CloudUploadIcon/>}
                       sx={{mr: "2rem", bgcolor: content ? "purple" : "blue"}}>
                   {content ? `${content.name.slice(0, 10)}...` : "Upload Content File"}
                   <VisuallyHiddenInput type="file" onChange={uploadFile(setContent)}/>
               </Button>

               <Button component="label" variant="contained" startIcon={<CloudUploadIcon/>}
                       sx={{mr: "2rem", bgcolor: publicKey ? "purple" : "blue"}}>
                   {publicKey ? `${publicKey.name.slice(0, 10)}...` : "Upload Public Key"}
                   <VisuallyHiddenInput type="file" onChange={uploadFile(setPublicKey)}/>
               </Button>

               <Button component="label" variant="contained" startIcon={<CloudUploadIcon/>}
                       sx={{mr: "2rem", bgcolor: signature ? "purple" : "blue"}}>
                   {signature ? `${signature.name.slice(0, 10)}...` : "Upload Signature"}
                   <VisuallyHiddenInput type="file" onChange={uploadFile(setSignature)}/>
               </Button>

               <Button variant="contained" onClick={verify}>Verify</Button>
           </Box>

            {success ? <Alert severity="success">{success}</Alert> : null}
            {error ? <Alert severity="error">{error}</Alert> : null}
        </Box>
    );
}

export {
    VerifyFrom
}