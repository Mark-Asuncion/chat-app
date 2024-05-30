import { useCallback, useEffect, useState } from 'react';
import { NavigateFunction, useNavigate } from "react-router-dom";
import env from "react-dotenv";

function App() {
    const navigate = useNavigate();

    const isLoggedIn = useCallback(async(navigate: NavigateFunction) => {
        const apiAuth = `${env.API_DOMAIN}/auth/login`;
        const res = await fetch(apiAuth, {
            method: "POST",
            credentials: 'include',
            headers: {
                "Content-Type": "application/json"
            }
        });
        // console.log(res);
        if (!res.ok) {
            return navigate("/login");
        }
    },[]);

    useEffect(() => {
        console.log("isLoggedIn");
        isLoggedIn(navigate);
    }, []);
    return (
        <p className='text-6xl text-white'>Home</p>
    );
}

export default App;
