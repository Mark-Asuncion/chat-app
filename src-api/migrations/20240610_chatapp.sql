--
-- PostgreSQL database cluster dump
--

-- Started on 2024-06-10 21:04:34 PST

SET default_transaction_read_only = off;

SET client_encoding = 'UTF8';
SET standard_conforming_strings = on;

--
-- Roles
--

CREATE ROLE client;
ALTER ROLE client WITH NOSUPERUSER INHERIT NOCREATEROLE NOCREATEDB LOGIN NOREPLICATION NOBYPASSRLS PASSWORD 'SCRAM-SHA-256$4096:nRB8bU2uHAoBKQT6g7eVuQ==$+nY7L3l2x4f7lJEa1tGQPZblcbvWktnZ8yKZjFJiWKk=:hPRJkHj8xpUWtbJCh+jQZ9hM2IVechrRWS0JPLzRlpg=';
CREATE ROLE postgres;
ALTER ROLE postgres WITH SUPERUSER INHERIT CREATEROLE CREATEDB LOGIN REPLICATION BYPASSRLS PASSWORD 'SCRAM-SHA-256$4096:LcWQd+rgjIY97TU3IES4LQ==$cOP4PcH8PLTVFvB3PJuF6iwN14kak0t/Gd/1T/iPXMQ=:wZAlWmajbK7Cn6lbsm0CuCY77QGtLGfnNH9y3v2fG9g=';

--
-- User Configurations
--








--
-- Databases
--

--
-- Database "template1" dump
--

\connect template1

--
-- PostgreSQL database dump
--

-- Dumped from database version 15.6 (Debian 15.6-0+deb12u1)
-- Dumped by pg_dump version 15.6 (Debian 15.6-0+deb12u1)

-- Started on 2024-06-10 21:04:34 PST

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

-- Completed on 2024-06-10 21:04:34 PST

--
-- PostgreSQL database dump complete
--

--
-- Database "ChatApp" dump
--

--
-- PostgreSQL database dump
--

-- Dumped from database version 15.6 (Debian 15.6-0+deb12u1)
-- Dumped by pg_dump version 15.6 (Debian 15.6-0+deb12u1)

-- Started on 2024-06-10 21:04:34 PST

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

--
-- TOC entry 3358 (class 1262 OID 16389)
-- Name: ChatApp; Type: DATABASE; Schema: -; Owner: postgres
--

CREATE DATABASE "ChatApp" WITH TEMPLATE = template0 ENCODING = 'UTF8' LOCALE_PROVIDER = libc LOCALE = 'en_PH.UTF-8';


ALTER DATABASE "ChatApp" OWNER TO postgres;

\connect "ChatApp"

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
    id character varying(36) NOT NULL,
    email character varying(255) NOT NULL,
    username character varying(255) NOT NULL,
    password character varying(255) NOT NULL
);


ALTER TABLE public.accounts OWNER TO postgres;

--
-- TOC entry 215 (class 1259 OID 24596)
-- Name: salts; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.salts (
    id character varying(255) NOT NULL,
    user_id character varying(36) NOT NULL
);


ALTER TABLE public.salts OWNER TO postgres;

--
-- TOC entry 3203 (class 2606 OID 32793)
-- Name: accounts _enq_email; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.accounts
    ADD CONSTRAINT _enq_email UNIQUE (email);


--
-- TOC entry 3205 (class 2606 OID 32795)
-- Name: accounts _enq_username; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.accounts
    ADD CONSTRAINT _enq_username UNIQUE (username);


--
-- TOC entry 3207 (class 2606 OID 24595)
-- Name: accounts accounts_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.accounts
    ADD CONSTRAINT accounts_pkey PRIMARY KEY (id);


--
-- TOC entry 3209 (class 2606 OID 24600)
-- Name: salts salts_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.salts
    ADD CONSTRAINT salts_pkey PRIMARY KEY (id);


--
-- TOC entry 3210 (class 2606 OID 24601)
-- Name: salts _user_id; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.salts
    ADD CONSTRAINT _user_id FOREIGN KEY (user_id) REFERENCES public.accounts(id);


--
-- TOC entry 3359 (class 0 OID 0)
-- Dependencies: 214
-- Name: TABLE accounts; Type: ACL; Schema: public; Owner: postgres
--

GRANT SELECT,INSERT,UPDATE ON TABLE public.accounts TO client;


--
-- TOC entry 3360 (class 0 OID 0)
-- Dependencies: 215
-- Name: TABLE salts; Type: ACL; Schema: public; Owner: postgres
--

GRANT SELECT,INSERT,DELETE,UPDATE ON TABLE public.salts TO client;


-- Completed on 2024-06-10 21:04:34 PST

--
-- PostgreSQL database dump complete
--

--
-- Database "postgres" dump
--

\connect postgres

--
-- PostgreSQL database dump
--

-- Dumped from database version 15.6 (Debian 15.6-0+deb12u1)
-- Dumped by pg_dump version 15.6 (Debian 15.6-0+deb12u1)

-- Started on 2024-06-10 21:04:34 PST

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

-- Completed on 2024-06-10 21:04:34 PST

--
-- PostgreSQL database dump complete
--

-- Completed on 2024-06-10 21:04:34 PST

--
-- PostgreSQL database cluster dump complete
--

