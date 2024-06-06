import { useCallback, useEffect, useState } from 'react';
import { NavigateFunction, useNavigate } from "react-router-dom";
import env from "react-dotenv";
import Pane from './components/Pane';
import Searchbar from './components/Searchbar';
import UsersList, { UserEntry } from './components/UsersList';

async function autoLogin(navigate: NavigateFunction, setLoggedIn: React.Dispatch<React.SetStateAction<boolean>>) {
    const apiAuth = `${env.API_DOMAIN}/auth/login`;
    const res = await fetch(apiAuth, {
        method: "POST",
        credentials: 'include',
        headers: {
            "Content-Type": "application/json"
        }
    });
    if (!res.ok) {
        // TODO show modal user session is invalid and redirect 
        return navigate("/login");
    }
    // remove this
    setLoggedIn(true);
}

function App() {
    const navigate = useNavigate();
    const [loggedIn, setLoggedIn] = useState(false);
    const [searchbar, setSearchbar] = useState("");

    const isLoggedIn = useCallback(autoLogin,[]);
    useEffect(() => {
        if (!loggedIn)
            isLoggedIn(navigate, setLoggedIn);
    }, []);

    const users: UserEntry[] = [
        {
            profileImg: "none",
            name: "Name",
            message: "message",
            lastSent: 30
        },
        {
            selected: true,
            profileImg: "none",
            name: "Name",
            message: "message",
            lastSent: 700
        }
    ]

    // TODO register listener shortcut
    return (
        <div className='flex flex-row gap-3 p-3 h-[100%]'>
            <Pane className='w-max'>
                <h1 className='text-white text-3xl font-bold'>Inbox</h1>
                <Searchbar
                    value={searchbar}
                    className='max-w-[32rem]'
                    placeholder='Search'
                    onChange={(e) => {
                        const v = e.target.value;
                        setSearchbar(v);
                    }}
                    shortcut="/"
                />
                <UsersList
                    className="mt-3"
                    entries={users}
                />
            </Pane>
            <Pane
                className='grow'>
            </Pane>
        </div>
    );
}

export default App;
