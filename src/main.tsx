import React from 'react';
import ReactDOM from 'react-dom/client';
import './styles/main.css';
import App from './modules/App';
import { Init as LoginRegister } from "./modules/Login";

import { createBrowserRouter, RouterProvider, } from "react-router-dom";

const root = ReactDOM.createRoot(
    document.getElementById('root') as HTMLElement
);
document.body.classList.add("bg-neutral-950");

const router = createBrowserRouter([
    {
        path: "/",
        element: <App />
    },
    {
        path: "/login",
        element: <LoginRegister />
    },
]);

root.render(
    <React.StrictMode>
        <RouterProvider router={router} />
    </React.StrictMode>
);
