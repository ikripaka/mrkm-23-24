import {AppBar, Box, Button, IconButton, Toolbar, Typography} from "@mui/material";


const Navbar = () => {
    return (
        <Box sx={{ flexGrow: 1 }}>
            <AppBar position="static">
                <Toolbar>
                    <IconButton
                        size="large"
                        edge="start"
                        color="inherit"
                        aria-label="menu"
                        sx={{ mr: 2 }}
                    >
                    </IconButton>
                    <Typography variant="h6" component="div" sx={{ flexGrow: 1 }}>
                        Check Your Sign
                    </Typography>
                </Toolbar>
            </AppBar>
        </Box>
    )
}

export {
    Navbar
}