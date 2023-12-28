import {Navbar} from "./components/Navbar";
import {Box, Container} from "@mui/material";
import {VerifyFrom} from "./components/VerifyForm";
import {RegisterForm} from "./components/RegisterForm";

function App() {
    return (
        <div className="App">
            <Navbar/>
            <Container fixed>
                <Box component="div"
                     sx={{ display: 'block', m: '2rem' }}
                >
                    <RegisterForm />
                    <VerifyFrom />
                </Box>
            </Container>
        </div>
    );
}

export default App;
