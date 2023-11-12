import {useState} from "react";
import * as React from "react";
import {Alert, Box, TextField} from "@mui/material";
import Button from "@mui/material/Button";
import CloudUploadIcon from "@mui/icons-material/CloudUpload";
import {styled} from "@mui/material/styles";

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

const RegisterForm = () => {
    const [nickname, setNickname] = useState("")
    const [publicKey, setPublicKey] = useState(null);

    const [success, setSuccess] = useState("");
    const [error, setError] = useState("")

    const uploadFile = (setter) => (e) => {
        setter(e.target.files[0])
    }

    const register = () => {
        const formData = new FormData();

        formData.append("public_key", publicKey);
        formData.append("nickname", nickname)

        fetch("/ds/register", { method: 'POST', body: formData })
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
            <h1>Register</h1>

            <Box sx={{mb: "2rem", display: "flex"}}>

                <TextField id="nickname" label="Nickname" variant="outlined"
                           sx={{mr: "2rem"}}
                           value={nickname} onChange={(e) => setNickname(e.target.value)} />

                <Button component="label" variant="contained" startIcon={<CloudUploadIcon/>}
                        sx={{mr: "2rem", bgcolor: publicKey ? "purple" : "blue"}}>
                    {publicKey ? `${publicKey.name.slice(0, 10)}...` : "Upload Public Key"}
                    <VisuallyHiddenInput type="file" onChange={uploadFile(setPublicKey)}/>
                </Button>

                <Button variant="contained" onClick={register}>Register</Button>
            </Box>

            {success ? <Alert severity="success">{success}</Alert> : null}
            {error ? <Alert severity="error">{error}</Alert> : null}
        </Box>
    )
}

export {
    RegisterForm
}