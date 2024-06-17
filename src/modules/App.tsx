import { useCallback, useEffect, useState } from 'react';
import { NavigateFunction as _, useNavigate } from "react-router-dom";
import Pane from './components/Pane';
import Searchbar from './components/Searchbar';
import UsersList, { UserEntry } from './components/UsersList';
import UserInfo, { IUserInfo } from './components/UserInfo';
import Loading from './components/Loading';

async function getUserInfo(cbOk: (v: IUserInfo) => void, cbFail: () => void) {
    const apiUser = `${import.meta.env.VITE_API_DOMAIN}/user/info`;
    const res = await fetch(apiUser, {
        credentials: 'include'
    });
    if (res.ok) {
        return cbOk(await res.json() as IUserInfo);
    }
    cbFail();
}

function App() {
    const navigate = useNavigate();
    const [searchbar, setSearchbar] = useState("");
    const [userinfo, setUserInfo] = useState<IUserInfo | null>(null);

    const gUserInfo = useCallback(() => getUserInfo(
        (v: IUserInfo) => {
            setUserInfo(v);
        },
        () => {
            // show notif user session is expired
            navigate("/login");
        }
    ), []);

    useEffect(() => {
        if (userinfo) {
            const gtUserInfo = setTimeout(gUserInfo, 2000);
            return () => clearTimeout(gtUserInfo);
        }
        else {
            gUserInfo();
        }
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

    if (userinfo == null) {
        return (
            <Loading />
        )
    }

    return (
        <div className='flex flex-row gap-3 p-3 h-[100%]'>
            <Pane className='w-max flex flex-col'>
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
                <div className='border-b border-neutral-600 mt-auto'></div>
                <UserInfo
                    profileImg="none"
                    name={( userinfo )? userinfo.username:"loading..."}
                />
            </Pane>
            <Pane
                className='grow'>
                {<p>test</p>}
            </Pane>
        </div>
    );
}

export default App;
