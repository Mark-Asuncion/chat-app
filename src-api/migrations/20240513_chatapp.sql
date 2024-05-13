--
-- PostgreSQL database dump
--

-- Dumped from database version 15.6 (Debian 15.6-0+deb12u1)
-- Dumped by pg_dump version 15.6 (Debian 15.6-0+deb12u1)

-- Started on 2024-05-13 21:52:14 PST

SET statement_timeout = 0;
SET lock_timeout = 0;
SET idle_in_transaction_session_timeout = 0;
SET client_encoding = 'UTF8';
SET standard_conforming_strings = on;
SELECT pg_catalog.set_config('search_path', '', false);
SET check_function_bodies = false;
SET xmloption = content;
SET client_min_messages = warning;
SET row_security = off;

SET default_tablespace = '';

SET default_table_access_method = heap;

--
-- TOC entry 214 (class 1259 OID 16397)
-- Name: accounts; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.accounts (
    id character varying(30) NOT NULL,
    email character varying(255) NOT NULL,
    username character varying(255) NOT NULL,
    password character varying(255) NOT NULL
);


ALTER TABLE public.accounts OWNER TO postgres;

--
-- TOC entry 3341 (class 0 OID 16397)
-- Dependencies: 214
-- Data for Name: accounts; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.accounts (id, email, username, password) FROM stdin;
\.


--
-- TOC entry 3198 (class 2606 OID 16403)
-- Name: accounts accounts_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.accounts
    ADD CONSTRAINT accounts_pkey PRIMARY KEY (id);


-- Completed on 2024-05-13 21:52:14 PST

--
-- PostgreSQL database dump complete
--

