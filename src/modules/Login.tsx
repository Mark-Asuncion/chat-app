import React, { useEffect } from 'react';
import { TabEntry, Tabs } from './Utils';
import env from "react-dotenv";

interface LoginData {
    email: string,
    password: string
};

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
        <p className='text-red-500 text-sm'>{ value }</p>
    )
}

function login() {};

function register() {};

export function Init() {
    const [isShowPass, setIsShowPass] = React.useState(false);
    const [isErr, setIsErr] = React.useState(false);
    const [isLogin, setIsLogin] = React.useState(true);
    const apiAuth = `${env.API_DOMAIN}/auth/login`;
    const tabEntries = [
        { active: true, text: "Login", cb: () => setIsLogin(true) },
        { text: "Register", cb: () => setIsLogin(false) },
    ];

    return (
        <div className='p-10 flex flex-col text-white w-1/2 mx-auto mt-20 bg-neutral-900
            rounded-md max-w-[475px] shadow-lg shadow-black'>
            <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" strokeWidth={1.5} stroke="currentColor" 
                className="mx-auto my-4 w-1/4 h-auto">
                <path strokeLinecap="round" strokeLinejoin="round" d="M20.25 8.511c.884.284 1.5 1.128 1.5 2.097v4.286c0 1.136-.847 2.1-1.98 2.193-.34.027-.68.052-1.02.072v3.091l-3-3c-1.354 0-2.694-.055-4.02-.163a2.115 2.115 0 0 1-.825-.242m9.345-8.334a2.126 2.126 0 0 0-.476-.095 48.64 48.64 0 0 0-8.048 0c-1.131.094-1.976 1.057-1.976 2.192v4.286c0 .837.46 1.58 1.155 1.951m9.345-8.334V6.637c0-1.621-1.152-3.026-2.76-3.235A48.455 48.455 0 0 0 11.25 3c-2.115 0-4.198.137-6.24.402-1.608.209-2.76 1.614-2.76 3.235v6.226c0 1.621 1.152 3.026 2.76 3.235.577.075 1.157.14 1.74.194V21l4.155-4.155" />
            </svg>
            <Tabs entries={tabEntries}/>
            <div>
                <label htmlFor="price" className="block text-sm font-medium leading-6">
                    Email
                </label>
                <div className="relative mt-2 rounded-md shadow-sm">
                    <input
                        type="email"
                        name="email"
                        id="email"
                        className="block bg-neutral-700 w-full rounded-md border-0 py-2 pl-3
                        ring-1 ring-inset ring-gray-300 placeholder:text-gray-400
                        focus:ring-2 focus:ring-inset focus:ring-accent focus:outline-none sm:text-sm sm:leading-6"
                        placeholder="email@domain.com"
                    />
                </div>
                { (isErr)? <ErrText value="Invalid email format"/>:null }
                <label htmlFor="price" className="block text-sm font-medium leading-6 mt-5">
                    Password
                </label>
                <div className="relative mt-2 rounded-md shadow-sm">
                    <input
                        type={(isShowPass)? "text":"password"}
                        name="password"
                        id="password"
                        className="block bg-neutral-700 w-full rounded-md border-0 py-2 pl-3
                        ring-1 ring-inset ring-gray-300 placeholder:text-gray-400
                        focus:ring-2 focus:ring-inset focus:ring-accent focus:outline-none sm:text-sm sm:leading-6"
                        placeholder="password"
                    />
                    <button
                        type='button'
                        onClick={() => setIsShowPass(!isShowPass)}
                        className='absolute inset-y-0 right-0 flex items-center pr-3 button'>
                        { (isShowPass)? <EyeOpen />:<EyeClose /> }
                    </button>
                </div>
                { (isErr)? <ErrText value="Password must be at least 8 characters"/>:null }
                <div className='border-b border-neutral-600 mt-14'></div>
                <button type='button'
                    onClick={() => {}}
                    className='text-center mx-auto p-3 bg-accent
                    w-full rounded-md font-bold my-4
                    hover:brightness-125'>Login</button>
            </div>
        </div>
    )
}
