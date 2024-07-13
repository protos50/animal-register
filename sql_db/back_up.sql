toc.dat                                                                                             0000600 0004000 0002000 00000033717 14642064323 0014456 0                                                                                                    ustar 00postgres                        postgres                        0000000 0000000                                                                                                                                                                        PGDMP   +    *                |            animal_register    16.3    16.3 3    y           0    0    ENCODING    ENCODING        SET client_encoding = 'UTF8';
                      false         z           0    0 
   STDSTRINGS 
   STDSTRINGS     (   SET standard_conforming_strings = 'on';
                      false         {           0    0 
   SEARCHPATH 
   SEARCHPATH     8   SELECT pg_catalog.set_config('search_path', '', false);
                      false         |           1262    16388    animal_register    DATABASE     {   CREATE DATABASE animal_register WITH TEMPLATE = template0 ENCODING = 'UTF8' LOCALE_PROVIDER = libc LOCALE = 'en_GB.UTF-8';
    DROP DATABASE animal_register;
                postgres    false         �            1259    16510    family    TABLE     b   CREATE TABLE public.family (
    id_family integer NOT NULL,
    scientific_name text NOT NULL
);
    DROP TABLE public.family;
       public         heap    postgres    false         �            1259    16509    family_id_family_seq    SEQUENCE     �   CREATE SEQUENCE public.family_id_family_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;
 +   DROP SEQUENCE public.family_id_family_seq;
       public          postgres    false    216         }           0    0    family_id_family_seq    SEQUENCE OWNED BY     M   ALTER SEQUENCE public.family_id_family_seq OWNED BY public.family.id_family;
          public          postgres    false    215         �            1259    16547    genus    TABLE        CREATE TABLE public.genus (
    id_genus integer NOT NULL,
    id_tribe integer NOT NULL,
    scientific_name text NOT NULL
);
    DROP TABLE public.genus;
       public         heap    postgres    false         �            1259    16546    genus_id_genus_seq    SEQUENCE     �   CREATE SEQUENCE public.genus_id_genus_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;
 )   DROP SEQUENCE public.genus_id_genus_seq;
       public          postgres    false    222         ~           0    0    genus_id_genus_seq    SEQUENCE OWNED BY     I   ALTER SEQUENCE public.genus_id_genus_seq OWNED BY public.genus.id_genus;
          public          postgres    false    221         �            1259    16575    observation    TABLE     j   CREATE TABLE public.observation (
    id_observation integer NOT NULL,
    id_species integer NOT NULL
);
    DROP TABLE public.observation;
       public         heap    postgres    false         �            1259    16574    observation_id_observation_seq    SEQUENCE     �   CREATE SEQUENCE public.observation_id_observation_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;
 5   DROP SEQUENCE public.observation_id_observation_seq;
       public          postgres    false    226                    0    0    observation_id_observation_seq    SEQUENCE OWNED BY     a   ALTER SEQUENCE public.observation_id_observation_seq OWNED BY public.observation.id_observation;
          public          postgres    false    225         �            1259    16561    species    TABLE     �   CREATE TABLE public.species (
    id_species integer NOT NULL,
    id_genus integer NOT NULL,
    scientific_name text NOT NULL
);
    DROP TABLE public.species;
       public         heap    postgres    false         �            1259    16560    species_id_species_seq    SEQUENCE     �   CREATE SEQUENCE public.species_id_species_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;
 -   DROP SEQUENCE public.species_id_species_seq;
       public          postgres    false    224         �           0    0    species_id_species_seq    SEQUENCE OWNED BY     Q   ALTER SEQUENCE public.species_id_species_seq OWNED BY public.species.id_species;
          public          postgres    false    223         �            1259    16519 	   subfamily    TABLE     �   CREATE TABLE public.subfamily (
    id_subfamily integer NOT NULL,
    id_family integer NOT NULL,
    scientific_name text NOT NULL
);
    DROP TABLE public.subfamily;
       public         heap    postgres    false         �            1259    16518    subfamily_id_subfamily_seq    SEQUENCE     �   CREATE SEQUENCE public.subfamily_id_subfamily_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;
 1   DROP SEQUENCE public.subfamily_id_subfamily_seq;
       public          postgres    false    218         �           0    0    subfamily_id_subfamily_seq    SEQUENCE OWNED BY     Y   ALTER SEQUENCE public.subfamily_id_subfamily_seq OWNED BY public.subfamily.id_subfamily;
          public          postgres    false    217         �            1259    16533    tribe    TABLE     �   CREATE TABLE public.tribe (
    id_tribe integer NOT NULL,
    id_subfamily integer NOT NULL,
    scientific_name text NOT NULL
);
    DROP TABLE public.tribe;
       public         heap    postgres    false         �            1259    16532    tribe_id_tribe_seq    SEQUENCE     �   CREATE SEQUENCE public.tribe_id_tribe_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;
 )   DROP SEQUENCE public.tribe_id_tribe_seq;
       public          postgres    false    220         �           0    0    tribe_id_tribe_seq    SEQUENCE OWNED BY     I   ALTER SEQUENCE public.tribe_id_tribe_seq OWNED BY public.tribe.id_tribe;
          public          postgres    false    219         �           2604    16513    family id_family    DEFAULT     t   ALTER TABLE ONLY public.family ALTER COLUMN id_family SET DEFAULT nextval('public.family_id_family_seq'::regclass);
 ?   ALTER TABLE public.family ALTER COLUMN id_family DROP DEFAULT;
       public          postgres    false    215    216    216         �           2604    16550    genus id_genus    DEFAULT     p   ALTER TABLE ONLY public.genus ALTER COLUMN id_genus SET DEFAULT nextval('public.genus_id_genus_seq'::regclass);
 =   ALTER TABLE public.genus ALTER COLUMN id_genus DROP DEFAULT;
       public          postgres    false    222    221    222         �           2604    16578    observation id_observation    DEFAULT     �   ALTER TABLE ONLY public.observation ALTER COLUMN id_observation SET DEFAULT nextval('public.observation_id_observation_seq'::regclass);
 I   ALTER TABLE public.observation ALTER COLUMN id_observation DROP DEFAULT;
       public          postgres    false    226    225    226         �           2604    16564    species id_species    DEFAULT     x   ALTER TABLE ONLY public.species ALTER COLUMN id_species SET DEFAULT nextval('public.species_id_species_seq'::regclass);
 A   ALTER TABLE public.species ALTER COLUMN id_species DROP DEFAULT;
       public          postgres    false    223    224    224         �           2604    16522    subfamily id_subfamily    DEFAULT     �   ALTER TABLE ONLY public.subfamily ALTER COLUMN id_subfamily SET DEFAULT nextval('public.subfamily_id_subfamily_seq'::regclass);
 E   ALTER TABLE public.subfamily ALTER COLUMN id_subfamily DROP DEFAULT;
       public          postgres    false    218    217    218         �           2604    16536    tribe id_tribe    DEFAULT     p   ALTER TABLE ONLY public.tribe ALTER COLUMN id_tribe SET DEFAULT nextval('public.tribe_id_tribe_seq'::regclass);
 =   ALTER TABLE public.tribe ALTER COLUMN id_tribe DROP DEFAULT;
       public          postgres    false    220    219    220         l          0    16510    family 
   TABLE DATA           <   COPY public.family (id_family, scientific_name) FROM stdin;
    public          postgres    false    216       4460.dat r          0    16547    genus 
   TABLE DATA           D   COPY public.genus (id_genus, id_tribe, scientific_name) FROM stdin;
    public          postgres    false    222       4466.dat v          0    16575    observation 
   TABLE DATA           A   COPY public.observation (id_observation, id_species) FROM stdin;
    public          postgres    false    226       4470.dat t          0    16561    species 
   TABLE DATA           H   COPY public.species (id_species, id_genus, scientific_name) FROM stdin;
    public          postgres    false    224       4468.dat n          0    16519 	   subfamily 
   TABLE DATA           M   COPY public.subfamily (id_subfamily, id_family, scientific_name) FROM stdin;
    public          postgres    false    218       4462.dat p          0    16533    tribe 
   TABLE DATA           H   COPY public.tribe (id_tribe, id_subfamily, scientific_name) FROM stdin;
    public          postgres    false    220       4464.dat �           0    0    family_id_family_seq    SEQUENCE SET     C   SELECT pg_catalog.setval('public.family_id_family_seq', 10, true);
          public          postgres    false    215         �           0    0    genus_id_genus_seq    SEQUENCE SET     @   SELECT pg_catalog.setval('public.genus_id_genus_seq', 9, true);
          public          postgres    false    221         �           0    0    observation_id_observation_seq    SEQUENCE SET     M   SELECT pg_catalog.setval('public.observation_id_observation_seq', 20, true);
          public          postgres    false    225         �           0    0    species_id_species_seq    SEQUENCE SET     D   SELECT pg_catalog.setval('public.species_id_species_seq', 8, true);
          public          postgres    false    223         �           0    0    subfamily_id_subfamily_seq    SEQUENCE SET     H   SELECT pg_catalog.setval('public.subfamily_id_subfamily_seq', 8, true);
          public          postgres    false    217         �           0    0    tribe_id_tribe_seq    SEQUENCE SET     A   SELECT pg_catalog.setval('public.tribe_id_tribe_seq', 12, true);
          public          postgres    false    219         �           2606    16517    family pk_family 
   CONSTRAINT     U   ALTER TABLE ONLY public.family
    ADD CONSTRAINT pk_family PRIMARY KEY (id_family);
 :   ALTER TABLE ONLY public.family DROP CONSTRAINT pk_family;
       public            postgres    false    216         �           2606    16554    genus pk_genus 
   CONSTRAINT     R   ALTER TABLE ONLY public.genus
    ADD CONSTRAINT pk_genus PRIMARY KEY (id_genus);
 8   ALTER TABLE ONLY public.genus DROP CONSTRAINT pk_genus;
       public            postgres    false    222         �           2606    16580    observation pk_observation 
   CONSTRAINT     d   ALTER TABLE ONLY public.observation
    ADD CONSTRAINT pk_observation PRIMARY KEY (id_observation);
 D   ALTER TABLE ONLY public.observation DROP CONSTRAINT pk_observation;
       public            postgres    false    226         �           2606    16568    species pk_species 
   CONSTRAINT     X   ALTER TABLE ONLY public.species
    ADD CONSTRAINT pk_species PRIMARY KEY (id_species);
 <   ALTER TABLE ONLY public.species DROP CONSTRAINT pk_species;
       public            postgres    false    224         �           2606    16526    subfamily pk_subfamily 
   CONSTRAINT     ^   ALTER TABLE ONLY public.subfamily
    ADD CONSTRAINT pk_subfamily PRIMARY KEY (id_subfamily);
 @   ALTER TABLE ONLY public.subfamily DROP CONSTRAINT pk_subfamily;
       public            postgres    false    218         �           2606    16540    tribe pk_tribe 
   CONSTRAINT     R   ALTER TABLE ONLY public.tribe
    ADD CONSTRAINT pk_tribe PRIMARY KEY (id_tribe);
 8   ALTER TABLE ONLY public.tribe DROP CONSTRAINT pk_tribe;
       public            postgres    false    220         �           2606    16555    genus fk_genus_tribe    FK CONSTRAINT     z   ALTER TABLE ONLY public.genus
    ADD CONSTRAINT fk_genus_tribe FOREIGN KEY (id_tribe) REFERENCES public.tribe(id_tribe);
 >   ALTER TABLE ONLY public.genus DROP CONSTRAINT fk_genus_tribe;
       public          postgres    false    220    222    4304         �           2606    16581 "   observation fk_observation_species    FK CONSTRAINT     �   ALTER TABLE ONLY public.observation
    ADD CONSTRAINT fk_observation_species FOREIGN KEY (id_species) REFERENCES public.species(id_species);
 L   ALTER TABLE ONLY public.observation DROP CONSTRAINT fk_observation_species;
       public          postgres    false    226    4308    224         �           2606    16569    species fk_species_genus    FK CONSTRAINT     ~   ALTER TABLE ONLY public.species
    ADD CONSTRAINT fk_species_genus FOREIGN KEY (id_genus) REFERENCES public.genus(id_genus);
 B   ALTER TABLE ONLY public.species DROP CONSTRAINT fk_species_genus;
       public          postgres    false    224    4306    222         �           2606    16527    subfamily fk_subfamily_family    FK CONSTRAINT     �   ALTER TABLE ONLY public.subfamily
    ADD CONSTRAINT fk_subfamily_family FOREIGN KEY (id_family) REFERENCES public.family(id_family);
 G   ALTER TABLE ONLY public.subfamily DROP CONSTRAINT fk_subfamily_family;
       public          postgres    false    216    218    4300         �           2606    16541    tribe fk_tribe_subfamily    FK CONSTRAINT     �   ALTER TABLE ONLY public.tribe
    ADD CONSTRAINT fk_tribe_subfamily FOREIGN KEY (id_subfamily) REFERENCES public.subfamily(id_subfamily);
 B   ALTER TABLE ONLY public.tribe DROP CONSTRAINT fk_tribe_subfamily;
       public          postgres    false    4302    218    220                                                         4460.dat                                                                                            0000600 0004000 0002000 00000000142 14642064323 0014250 0                                                                                                    ustar 00postgres                        postgres                        0000000 0000000                                                                                                                                                                        1	Formicidae
4	Nueva Familia 2
5	familia2
6	familia3
7	jkhkjhkj
8	femilia4
9	femilia5
10	asd
\.


                                                                                                                                                                                                                                                                                                                                                                                                                              4466.dat                                                                                            0000600 0004000 0002000 00000000174 14642064323 0014263 0                                                                                                    ustar 00postgres                        postgres                        0000000 0000000                                                                                                                                                                        1	1	Prionopelta
2	2	Hypoponera
3	3	Octostruma
4	4	Solenopsis
6	4	Nuevo Genero 1
7	3	genero2
8	12	Brachymyrmex
9	9	jedj
\.


                                                                                                                                                                                                                                                                                                                                                                                                    4470.dat                                                                                            0000600 0004000 0002000 00000000114 14642064323 0014250 0                                                                                                    ustar 00postgres                        postgres                        0000000 0000000                                                                                                                                                                        6	1
7	2
8	3
9	4
10	8
11	1
12	1
13	8
14	1
15	1
16	1
17	8
18	1
19	8
20	8
\.


                                                                                                                                                                                                                                                                                                                                                                                                                                                    4468.dat                                                                                            0000600 0004000 0002000 00000000162 14642064323 0014262 0                                                                                                    ustar 00postgres                        postgres                        0000000 0000000                                                                                                                                                                        1	1	Prionopelta punctulata
2	2	Hypoponera sp1
3	3	Octostruma balzani
4	4	Solenopsis sp1
8	8	Brachymyrmex sp1
\.


                                                                                                                                                                                                                                                                                                                                                                                                              4462.dat                                                                                            0000600 0004000 0002000 00000000154 14642064323 0014255 0                                                                                                    ustar 00postgres                        postgres                        0000000 0000000                                                                                                                                                                        1	1	Amblyoponinae
2	1	Ponerinae
3	1	Myrmicinae
5	1	subfamilia1
6	1	Formicinae
7	1	Ectatomminae
8	1	aaa
\.


                                                                                                                                                                                                                                                                                                                                                                                                                    4464.dat                                                                                            0000600 0004000 0002000 00000000231 14642064323 0014253 0                                                                                                    ustar 00postgres                        postgres                        0000000 0000000                                                                                                                                                                        1	1	Amblyoponini
2	2	Ponerini
3	3	Attini
4	3	Solenopsidini
6	5	tribu1
7	2	tribu1
8	2	tribu1
9	2	tribu1
10	1	tribu1
11	1	tribu2
12	6	Myrmelachistini
\.


                                                                                                                                                                                                                                                                                                                                                                       restore.sql                                                                                         0000600 0004000 0002000 00000026451 14642064323 0015400 0                                                                                                    ustar 00postgres                        postgres                        0000000 0000000                                                                                                                                                                        --
-- NOTE:
--
-- File paths need to be edited. Search for $$PATH$$ and
-- replace it with the path to the directory containing
-- the extracted data files.
--
--
-- PostgreSQL database dump
--

-- Dumped from database version 16.3
-- Dumped by pg_dump version 16.3

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

DROP DATABASE animal_register;
--
-- Name: animal_register; Type: DATABASE; Schema: -; Owner: postgres
--

CREATE DATABASE animal_register WITH TEMPLATE = template0 ENCODING = 'UTF8' LOCALE_PROVIDER = libc LOCALE = 'en_GB.UTF-8';


ALTER DATABASE animal_register OWNER TO postgres;

\connect animal_register

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
-- Name: family; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.family (
    id_family integer NOT NULL,
    scientific_name text NOT NULL
);


ALTER TABLE public.family OWNER TO postgres;

--
-- Name: family_id_family_seq; Type: SEQUENCE; Schema: public; Owner: postgres
--

CREATE SEQUENCE public.family_id_family_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER SEQUENCE public.family_id_family_seq OWNER TO postgres;

--
-- Name: family_id_family_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: postgres
--

ALTER SEQUENCE public.family_id_family_seq OWNED BY public.family.id_family;


--
-- Name: genus; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.genus (
    id_genus integer NOT NULL,
    id_tribe integer NOT NULL,
    scientific_name text NOT NULL
);


ALTER TABLE public.genus OWNER TO postgres;

--
-- Name: genus_id_genus_seq; Type: SEQUENCE; Schema: public; Owner: postgres
--

CREATE SEQUENCE public.genus_id_genus_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER SEQUENCE public.genus_id_genus_seq OWNER TO postgres;

--
-- Name: genus_id_genus_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: postgres
--

ALTER SEQUENCE public.genus_id_genus_seq OWNED BY public.genus.id_genus;


--
-- Name: observation; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.observation (
    id_observation integer NOT NULL,
    id_species integer NOT NULL
);


ALTER TABLE public.observation OWNER TO postgres;

--
-- Name: observation_id_observation_seq; Type: SEQUENCE; Schema: public; Owner: postgres
--

CREATE SEQUENCE public.observation_id_observation_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER SEQUENCE public.observation_id_observation_seq OWNER TO postgres;

--
-- Name: observation_id_observation_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: postgres
--

ALTER SEQUENCE public.observation_id_observation_seq OWNED BY public.observation.id_observation;


--
-- Name: species; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.species (
    id_species integer NOT NULL,
    id_genus integer NOT NULL,
    scientific_name text NOT NULL
);


ALTER TABLE public.species OWNER TO postgres;

--
-- Name: species_id_species_seq; Type: SEQUENCE; Schema: public; Owner: postgres
--

CREATE SEQUENCE public.species_id_species_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER SEQUENCE public.species_id_species_seq OWNER TO postgres;

--
-- Name: species_id_species_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: postgres
--

ALTER SEQUENCE public.species_id_species_seq OWNED BY public.species.id_species;


--
-- Name: subfamily; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.subfamily (
    id_subfamily integer NOT NULL,
    id_family integer NOT NULL,
    scientific_name text NOT NULL
);


ALTER TABLE public.subfamily OWNER TO postgres;

--
-- Name: subfamily_id_subfamily_seq; Type: SEQUENCE; Schema: public; Owner: postgres
--

CREATE SEQUENCE public.subfamily_id_subfamily_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER SEQUENCE public.subfamily_id_subfamily_seq OWNER TO postgres;

--
-- Name: subfamily_id_subfamily_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: postgres
--

ALTER SEQUENCE public.subfamily_id_subfamily_seq OWNED BY public.subfamily.id_subfamily;


--
-- Name: tribe; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.tribe (
    id_tribe integer NOT NULL,
    id_subfamily integer NOT NULL,
    scientific_name text NOT NULL
);


ALTER TABLE public.tribe OWNER TO postgres;

--
-- Name: tribe_id_tribe_seq; Type: SEQUENCE; Schema: public; Owner: postgres
--

CREATE SEQUENCE public.tribe_id_tribe_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER SEQUENCE public.tribe_id_tribe_seq OWNER TO postgres;

--
-- Name: tribe_id_tribe_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: postgres
--

ALTER SEQUENCE public.tribe_id_tribe_seq OWNED BY public.tribe.id_tribe;


--
-- Name: family id_family; Type: DEFAULT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.family ALTER COLUMN id_family SET DEFAULT nextval('public.family_id_family_seq'::regclass);


--
-- Name: genus id_genus; Type: DEFAULT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.genus ALTER COLUMN id_genus SET DEFAULT nextval('public.genus_id_genus_seq'::regclass);


--
-- Name: observation id_observation; Type: DEFAULT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.observation ALTER COLUMN id_observation SET DEFAULT nextval('public.observation_id_observation_seq'::regclass);


--
-- Name: species id_species; Type: DEFAULT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.species ALTER COLUMN id_species SET DEFAULT nextval('public.species_id_species_seq'::regclass);


--
-- Name: subfamily id_subfamily; Type: DEFAULT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.subfamily ALTER COLUMN id_subfamily SET DEFAULT nextval('public.subfamily_id_subfamily_seq'::regclass);


--
-- Name: tribe id_tribe; Type: DEFAULT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.tribe ALTER COLUMN id_tribe SET DEFAULT nextval('public.tribe_id_tribe_seq'::regclass);


--
-- Data for Name: family; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.family (id_family, scientific_name) FROM stdin;
\.
COPY public.family (id_family, scientific_name) FROM '$$PATH$$/4460.dat';

--
-- Data for Name: genus; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.genus (id_genus, id_tribe, scientific_name) FROM stdin;
\.
COPY public.genus (id_genus, id_tribe, scientific_name) FROM '$$PATH$$/4466.dat';

--
-- Data for Name: observation; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.observation (id_observation, id_species) FROM stdin;
\.
COPY public.observation (id_observation, id_species) FROM '$$PATH$$/4470.dat';

--
-- Data for Name: species; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.species (id_species, id_genus, scientific_name) FROM stdin;
\.
COPY public.species (id_species, id_genus, scientific_name) FROM '$$PATH$$/4468.dat';

--
-- Data for Name: subfamily; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.subfamily (id_subfamily, id_family, scientific_name) FROM stdin;
\.
COPY public.subfamily (id_subfamily, id_family, scientific_name) FROM '$$PATH$$/4462.dat';

--
-- Data for Name: tribe; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.tribe (id_tribe, id_subfamily, scientific_name) FROM stdin;
\.
COPY public.tribe (id_tribe, id_subfamily, scientific_name) FROM '$$PATH$$/4464.dat';

--
-- Name: family_id_family_seq; Type: SEQUENCE SET; Schema: public; Owner: postgres
--

SELECT pg_catalog.setval('public.family_id_family_seq', 10, true);


--
-- Name: genus_id_genus_seq; Type: SEQUENCE SET; Schema: public; Owner: postgres
--

SELECT pg_catalog.setval('public.genus_id_genus_seq', 9, true);


--
-- Name: observation_id_observation_seq; Type: SEQUENCE SET; Schema: public; Owner: postgres
--

SELECT pg_catalog.setval('public.observation_id_observation_seq', 20, true);


--
-- Name: species_id_species_seq; Type: SEQUENCE SET; Schema: public; Owner: postgres
--

SELECT pg_catalog.setval('public.species_id_species_seq', 8, true);


--
-- Name: subfamily_id_subfamily_seq; Type: SEQUENCE SET; Schema: public; Owner: postgres
--

SELECT pg_catalog.setval('public.subfamily_id_subfamily_seq', 8, true);


--
-- Name: tribe_id_tribe_seq; Type: SEQUENCE SET; Schema: public; Owner: postgres
--

SELECT pg_catalog.setval('public.tribe_id_tribe_seq', 12, true);


--
-- Name: family pk_family; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.family
    ADD CONSTRAINT pk_family PRIMARY KEY (id_family);


--
-- Name: genus pk_genus; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.genus
    ADD CONSTRAINT pk_genus PRIMARY KEY (id_genus);


--
-- Name: observation pk_observation; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.observation
    ADD CONSTRAINT pk_observation PRIMARY KEY (id_observation);


--
-- Name: species pk_species; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.species
    ADD CONSTRAINT pk_species PRIMARY KEY (id_species);


--
-- Name: subfamily pk_subfamily; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.subfamily
    ADD CONSTRAINT pk_subfamily PRIMARY KEY (id_subfamily);


--
-- Name: tribe pk_tribe; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.tribe
    ADD CONSTRAINT pk_tribe PRIMARY KEY (id_tribe);


--
-- Name: genus fk_genus_tribe; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.genus
    ADD CONSTRAINT fk_genus_tribe FOREIGN KEY (id_tribe) REFERENCES public.tribe(id_tribe);


--
-- Name: observation fk_observation_species; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.observation
    ADD CONSTRAINT fk_observation_species FOREIGN KEY (id_species) REFERENCES public.species(id_species);


--
-- Name: species fk_species_genus; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.species
    ADD CONSTRAINT fk_species_genus FOREIGN KEY (id_genus) REFERENCES public.genus(id_genus);


--
-- Name: subfamily fk_subfamily_family; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.subfamily
    ADD CONSTRAINT fk_subfamily_family FOREIGN KEY (id_family) REFERENCES public.family(id_family);


--
-- Name: tribe fk_tribe_subfamily; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.tribe
    ADD CONSTRAINT fk_tribe_subfamily FOREIGN KEY (id_subfamily) REFERENCES public.subfamily(id_subfamily);


--
-- PostgreSQL database dump complete
--

                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                       