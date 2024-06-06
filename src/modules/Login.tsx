import React, { useEffect, useRef } from 'react';
import { TabEntry, Tabs } from './Utils';
import env from "react-dotenv";
import { useNavigate } from 'react-router-dom';

function EyeClose() {
    return (
        <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" strokeWidth={1.5} stroke="currentColor" 
            className="w-6 h-6">
            <path strokeLinecap="round" strokeLinejoin="round" d="M3.98 8.223A10.477 10.477 0 0 0 1.934 12C3.226 16.338 7.244 19.5 12 19.5c.993 0 1.953-.138 2.863-.395M6.228 6.228A10.451 10.451 0 0 1 12 4.5c4.756 0 8.773 3.162 10.065 7.498a10.522 10.522 0 0 1-4.293 5.774M6.228 6.228 3 3m3.228 3.228 3.65 3.65m7.894 7.894L21 21m-3.228-3.228-3.65-3.65m0 0a3 3 0 1 0-4.243-4.243m4.242 4.242L9.88 9.88" />
        </svg>
    )
}

function EyeOpen() {
    return (
        <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" strokeWidth={1.5} stroke="currentColor" 
            className="w-6 h-6">
            <path strokeLinecap="round" strokeLinejoin="round" d="M2.036 12.322a1.012 1.012 0 0 1 0-.639C3.423 7.51 7.36 4.5 12 4.5c4.638 0 8.573 3.007 9.963 7.178.07.207.07.431 0 .639C20.577 16.49 16.64 19.5 12 19.5c-4.638 0-8.573-3.007-9.963-7.178Z" />
            <path strokeLinecap="round" strokeLinejoin="round" d="M15 12a3 3 0 1 1-6 0 3 3 0 0 1 6 0Z" />
        </svg>
    )
}

function ErrText({ value }:{ value: string }) {
    return (
        <p className='text-red-500 text-sm my-1'>{ value }</p>
    )
}

interface InputProps {
    value:                string,
    type?:                string,
    placeholder:          string,
    label:                string,
    isErr:                boolean,
    isErrText:            string,
    nameId:               string,
    containerclass?:      string,
    onChange:             (v: string) => void,
    getRef:               (el: HTMLInputElement) => void,
    onEnter:              (el: HTMLInputElement) => void
}

function Input(prop: InputProps) {
    return (
        <div className={( prop.containerclass )? prop.containerclass:""}>
            <label htmlFor={prop.nameId}
                className="block text-sm font-medium leading-6">
                {prop.label}
            </label>
            <div className="relative mt-2 rounded-md shadow-sm">
                <input
                    ref={el => {
                        if (el)
                            prop.getRef(el)
                    }}
                    type={( prop.type )? prop.type:"text"}
                    name={prop.nameId}
                    id={prop.nameId}
                    className="block bg-neutral-700 w-full rounded-md border-0 py-2 pl-3
                    ring-1 ring-inset ring-gray-300 placeholder:text-gray-400
                    focus:ring-2 focus:ring-inset focus:ring-accent focus:outline-none sm:text-sm sm:leading-6"
                    placeholder={prop.placeholder}
                    onKeyDown={(e) => {
                        if (e.key == "Enter") {
                            prop.onEnter(e.target as HTMLInputElement);
                        }
                    }}
                    onChange={ (e) => {
                        prop.onChange(e.target.value);
                    }}
                    value={prop.value}
                />
            </div>
        { (prop.isErr)? <ErrText value={prop.isErrText}/>:<></> }
        </div>
    )
}

function InputPassword(prop: InputProps) {
    const [isShowPass, setIsShowPass] = React.useState(false);
    return (
        <div className={( prop.containerclass )? prop.containerclass:""}>
            <label htmlFor={prop.nameId}
                className="block text-sm font-medium leading-6">
                {prop.label}
            </label>
            <div className="relative mt-2 rounded-md shadow-sm">
                <input
                    ref={el => {
                        if (el)
                            prop.getRef(el)
                    }}
                    type={ (isShowPass)? "text":"password" }
                    name={prop.nameId}
                    id={prop.nameId}
                    className="block bg-neutral-700 w-full rounded-md border-0 py-2 pl-3
                    ring-1 ring-inset ring-gray-300 placeholder:text-gray-400
                    focus:ring-2 focus:ring-inset focus:ring-accent focus:outline-none sm:text-sm sm:leading-6"
                    placeholder={prop.placeholder}
                    onKeyDown={(e) => {
                        if (e.key == "Enter") {
                            prop.onEnter(e.target as HTMLInputElement);
                        }
                    }}
                    onChange={ (e) => {
                        prop.onChange(e.target.value);
                    }}
                    value={prop.value}
                />
                <button
                    type='button'
                    onClick={() => setIsShowPass(!isShowPass)}
                    className='absolute inset-y-0 right-0 flex items-center pr-3'>
                    { (isShowPass)? <EyeOpen />:<EyeClose /> }
                </button>
            </div>
        { (prop.isErr)? <ErrText value={prop.isErrText}/>:<></> }
        </div>
    )
}

async function login_session(cbOk: () => void) {
    const apiAuth = `${env.API_DOMAIN}/auth/login`;
    const res = await fetch(apiAuth, {
        method: "POST",
        credentials: "include",
        headers: {
            "Content-Type": "application/json"
        }
    });
    if (res.ok) {
        cbOk();
    }
}

interface LoginRegisterInfo {
    email:    string | null,
    username: string | null,
    password: string
}

async function login(input: LoginRegisterInfo, cbOk: () => void, cbFail: () => void, validator: () => Promise<boolean>) {
    const apiAuth = `${env.API_DOMAIN}/auth/login`;

    if (!(await validator()) ) {
        return;
    }

    let isEmail = false;
    input.email = input.email!.trim();
    if (input.email!.match(/@/) != null) {
        isEmail = true;
    }

    const res = await fetch(apiAuth, {
        method: "POST",
        credentials: 'include',
        headers: {
            "Content-Type": "application/json"
        },
        body: JSON.stringify({
            email: (isEmail)? input.email:null,
            username: (!isEmail)? input.email:null,
            password: input.password
        })
    });
    if (res.ok) {
        return cbOk();
    }
    cbFail();
};

async function register(input: LoginRegisterInfo, cbOk: () => void, cbFail: () => void, validator: () => Promise<boolean>) {
    const apiAuth = `${env.API_DOMAIN}/auth/register`;
    const email = input.email?.trim();
    const username = input.username?.trim();
    const password = input.password.trim();

    if (!(await validator())) {
        return;
    }

    const res = await fetch(apiAuth, {
        method: "POST",
        credentials: 'include',
        headers: {
            "Content-Type": "application/json"
        },
        body: JSON.stringify({
            email,
            username,
            password
        })
    });
    if (res.ok) {
        return cbOk();
    }
    cbFail();
};

interface ErrState {
    email:    [ boolean, string ],
    username: [ boolean, string ],
    password: [ boolean, string ]
}

export function Init() {
    const navigate = useNavigate();
    const [sessionLogin, setSessionLoginStatus] = React.useState(true);
    const inputs = useRef<HTMLInputElement[]>([]);
    const loginBtnRef = useRef<HTMLButtonElement | null>(null);
    const [isErr, setIsErr] = React.useState<ErrState>({
        email:    [ false, "" ],
        username: [ false, "" ],
        password: [ false, "" ]
    });
    const [formsValue, setFormsValue] = React.useState({
        email: "",
        username: "",
        password: ""
    });

    async function validateEmailUsername(): Promise<boolean> {
        console.log("validateEmailUsername");
        const email = formsValue.email.trim();
        const username = formsValue.username.trim();
        let isNotErr = true;
        const nerr: {
            email: [ boolean, string ],
            username: [ boolean, string ],
        } = {
            email: [ false, "" ],
            username: [ false, "" ],
        };
        if (email.match(/\s/) != null) {
            nerr.email = [ true, "Email must not contain any spaces"];
            isNotErr = false;
        }

        if (username.match(/\s/) != null) {
            nerr.username = [ true, "Username must not contain any spaces"];
            isNotErr = false;
        }
        if (!isNotErr) {
            setIsErr(prev => {
                return {
                    ...prev,
                    email: nerr.email,
                    username: nerr.username
                };
            });
            return isNotErr;
        }

        if (tabEntries[1].active !== true) {
            return true;
        }

        const apiAuth = `${env.API_DOMAIN}/auth/validate`;
        const res = await fetch(apiAuth, {
            method: "POST",
            headers: {
                "Content-Type": "application/json"
            },
            body: JSON.stringify({
                email: ( formsValue.email.length == 0 )? null:formsValue.email,
                username: ( formsValue.username.length == 0 )? null:formsValue.username
            })
        });
        if (!res.ok) {
            return false;
        }

        const body = await res.json();
        if (body.email && body.email.length != 0) {
            nerr.email = [ true, (tabEntries[1].active)? "Email already exists":"" ];
            isNotErr = false;
        }
        if (body.username && body.username.length != 0) {
            nerr.username = [ true, "Username already exists" ];
            isNotErr = false;
        }
        if (!isNotErr)
            setIsErr(prev => {
                return {
                    ...prev,
                    email: nerr.email,
                    username: nerr.username
                };
            });
        return isNotErr;
    }

    async function validatePass(): Promise<boolean> {
        if (formsValue.password.length == 0) {
            return false;
        }
        const pass = formsValue.password.trim();
        if ( pass.length < 8 || pass.match(/\s/) != null) {
            setIsErr(prev => {
                return {
                    ...prev,
                    password: [ true, "Password must be at least 8 characters long and must not contain any space" ],
                }
            });
            return false;
        }
        else {
            setIsErr(prev => {
                return {
                    ...prev,
                    password: [ false, "" ],
                }
            });
        }
        return true;
    }

    function setTab(index: number, setTabEntries: React.Dispatch<React.SetStateAction<TabEntry[]>>) {
        setTabEntries(prev => {
            return prev.map((v, i) => {
                const active = i == index;
                return {
                    ...v,
                    active,
                };
            });
        });
        setFormsValue({
            email: "",
            username: "",
            password: ""
        });
        setIsErr({
            email: [ false, ""],
            username: [false, ""],
            password: [false, ""]
        });
    }

    inputs.current = [];
    function onEnter(el: HTMLInputElement) {
        let shouldMove = false;
        // console.log(inputs.current)
        for (let i=0;i<inputs.current.length;i++) {
            const v = inputs.current[i];
            if (shouldMove) {
                if (v.value.length != 0) {
                    break;
                }
                v.focus();
                return;
            }
            if (el == v) {
                shouldMove = true;
            }
        }
        if (loginBtnRef.current)
            loginBtnRef.current.click();
    }

    const [ tabEntries, setTabEntries ] = React.useState<TabEntry[]>([
        { active: true, text: "Login", cb: function() {
            setTab(0, setTabEntries);
        }
        },
        { text: "Register", cb: function() {
            setTab(1, setTabEntries);
        }
        },
    ]);

    useEffect(() => {
        if (sessionLogin) {
            setSessionLoginStatus(false);
            login_session(() => {
                navigate("/");
            });
        }
        const validateWithDelay = setTimeout(validateEmailUsername, 1000);
        validatePass();

        return () => clearTimeout(validateWithDelay);
    },[formsValue]);

    return (
        <div className='p-10 flex flex-col text-white w-1/2 mx-auto mt-10 bg-neutral-900
            rounded-md max-w-[475px] shadow-lg shadow-black'>
            <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" strokeWidth={1.5} stroke="currentColor"
                className="mx-auto my-4 w-1/4 h-auto">
                <path strokeLinecap="round" strokeLinejoin="round" d="M20.25 8.511c.884.284 1.5 1.128 1.5 2.097v4.286c0 1.136-.847 2.1-1.98 2.193-.34.027-.68.052-1.02.072v3.091l-3-3c-1.354 0-2.694-.055-4.02-.163a2.115 2.115 0 0 1-.825-.242m9.345-8.334a2.126 2.126 0 0 0-.476-.095 48.64 48.64 0 0 0-8.048 0c-1.131.094-1.976 1.057-1.976 2.192v4.286c0 .837.46 1.58 1.155 1.951m9.345-8.334V6.637c0-1.621-1.152-3.026-2.76-3.235A48.455 48.455 0 0 0 11.25 3c-2.115 0-4.198.137-6.24.402-1.608.209-2.76 1.614-2.76 3.235v6.226c0 1.621 1.152 3.026 2.76 3.235.577.075 1.157.14 1.74.194V21l4.155-4.155" />
            </svg>
            <Tabs entries={tabEntries}/>
            <div>
                <Input
                    value={formsValue.email}
                    type={(tabEntries[1].active)? "email":"text"}
                    placeholder={(tabEntries[1].active)? "Enter Email":"Enter Email or Username"}
                    label={ (tabEntries[1].active)? "Email":"Email or Username" }
                    isErr={isErr.email[0]}
                    isErrText={isErr.email[1]}
                    nameId="emailorusername"
                    getRef={el => inputs.current.push(el)}
                    onEnter={el => onEnter(el)}
                    onChange={(v) => {
                        setFormsValue(prev =>{
                            return {
                                ...prev,
                                email: v
                            }
                        });
                    }}
                />
                {
                    ( tabEntries[1].active )?
                        <Input
                            getRef={el => inputs.current.push(el)}
                            value={formsValue.username}
                            type="text"
                            placeholder="Enter Username"
                            label="username"
                            isErr={isErr.username[0]}
                            isErrText={isErr.username[1]}
                            nameId="username"
                            containerclass="mt-2"
                            onEnter={el => onEnter(el)}
                            onChange={(v) => {
                                setFormsValue(prev =>{
                                    return {
                                        ...prev,
                                        username: v
                                    }
                                });
                            }}
                        />:<></>
                }
                <InputPassword
                    getRef={el => inputs.current.push(el)}
                    value={formsValue.password}
                    placeholder="Enter Password"
                    label="password"
                    isErr={isErr.password[0]}
                    isErrText={isErr.password[1]}
                    nameId="password"
                    containerclass="mt-2"
                    onEnter={el => onEnter(el)}
                    onChange={(v) => {
                        setFormsValue(prev =>{
                            return {
                                ...prev,
                                password: v
                            }
                        });
                    }}
                />
                <div className='border-b border-neutral-600 mt-10'></div>
                <button type='button'
                    ref={loginBtnRef}
                    onClick={() => {
                        if (tabEntries[1].active) {
                            register(formsValue,
                                () => {
                                    setTabEntries(prev => {
                                        prev[0].active = true;
                                        prev[1].active = false;
                                        return prev;
                                    });
                                    setFormsValue({
                                        email: "",
                                        username: "",
                                        password: "",
                                    })
                                },
                                () => {
                                    setIsErr(prev => {
                                        return {
                                            username: [ true, "A Problem occured Please try again"],
                                            email: [ true, "A Problem occured Please try again"],
                                            password: [ true, "A Problem occured Please try again"]
                                        };
                                    })
                                },
                                async () => {
                                    return await validateEmailUsername() && await validatePass();
                                }
                            );
                        }
                        else {
                            login({
                                ...formsValue,
                                username: null
                            },
                                () => {
                                    navigate("/");
                                },
                                () => {
                                    setIsErr(prev => {
                                        return {
                                            ...prev,
                                            email: [ true, "The email you entered does not match any account."],
                                            password: [ true, "The password you entered is incorrect."]
                                        };
                                    })
                                },
                                async () => {
                                    return await validateEmailUsername() && await validatePass();
                                }
                            );
                        }
                    }}
                    className='text-center mx-auto p-3 bg-accent
                    w-full rounded-md font-bold my-4
                    hover:brightness-125'>
                    { (tabEntries[1].active)? "Register":"Login" }
                </button>
            </div>
        </div>
    )
}
