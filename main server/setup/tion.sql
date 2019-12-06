-- phpMyAdmin SQL Dump
-- version 4.9.0.1
-- https://www.phpmyadmin.net/
--
-- Host: 127.0.0.1
-- Generation Time: Dec 06, 2019 at 10:59 AM
-- Server version: 10.4.6-MariaDB
-- PHP Version: 7.3.9

SET SQL_MODE = "NO_AUTO_VALUE_ON_ZERO";
SET AUTOCOMMIT = 0;
START TRANSACTION;
SET time_zone = "+00:00";


/*!40101 SET @OLD_CHARACTER_SET_CLIENT=@@CHARACTER_SET_CLIENT */;
/*!40101 SET @OLD_CHARACTER_SET_RESULTS=@@CHARACTER_SET_RESULTS */;
/*!40101 SET @OLD_COLLATION_CONNECTION=@@COLLATION_CONNECTION */;
/*!40101 SET NAMES utf8mb4 */;

--
-- Database: `tion`
--

-- --------------------------------------------------------

--
-- Table structure for table `devices`
--

CREATE TABLE `devices` (
  `id` int(10) NOT NULL,
  `security_pool` int(10) NOT NULL,
  `devicename` varchar(255) NOT NULL,
  `device_type` enum('Camera','Windowsensor','Doorsensor','Motionsensor') NOT NULL,
  `room` varchar(255) NOT NULL,
  `token` varchar(255) NOT NULL
) ENGINE=InnoDB DEFAULT CHARSET=latin1;

-- --------------------------------------------------------

--
-- Table structure for table `security_pool`
--

CREATE TABLE `security_pool` (
  `id` int(10) NOT NULL,
  `name` varchar(255) NOT NULL,
  `owner` int(10) NOT NULL
) ENGINE=InnoDB DEFAULT CHARSET=latin1;

--
-- Dumping data for table `security_pool`
--

INSERT INTO `security_pool` (`id`, `name`, `owner`) VALUES
(1, 'sec_pool_1', 20);

-- --------------------------------------------------------

--
-- Table structure for table `users`
--

CREATE TABLE `users` (
  `id` int(10) NOT NULL,
  `security_pool` int(10) NOT NULL,
  `username` varchar(255) NOT NULL,
  `password` text NOT NULL,
  `email` varchar(255) NOT NULL,
  `role` enum('Admin','User','Device') NOT NULL,
  `token` varchar(255) NOT NULL
) ENGINE=InnoDB DEFAULT CHARSET=latin1;

--
-- Dumping data for table `users`
--

INSERT INTO `users` (`id`, `security_pool`, `username`, `password`, `email`, `role`, `token`) VALUES
(20, 1, 'username11', '$2b$12$EItZoFC7dDdaZeCX.fElf.RC19nZp3s9f80b.apct3Pu4c947AkR6', 'hey@hoi.nl11', 'Admin', 'oLZTFdV6HPMPLfGH5VD8QC22uEXeuv6UQgC1EaXHjVVky8GE1zwBmh2FgiDUotYuLNM9Cpsik33riuhVFdVX2xcwZnxbneTP3GXW2bPl4kWOCb72XpxzPQoiExvtSElB');

--
-- Indexes for dumped tables
--

--
-- Indexes for table `devices`
--
ALTER TABLE `devices`
  ADD PRIMARY KEY (`id`);

--
-- Indexes for table `security_pool`
--
ALTER TABLE `security_pool`
  ADD PRIMARY KEY (`id`),
  ADD UNIQUE KEY `name` (`name`);

--
-- Indexes for table `users`
--
ALTER TABLE `users`
  ADD PRIMARY KEY (`id`),
  ADD UNIQUE KEY `username` (`username`),
  ADD UNIQUE KEY `email` (`email`),
  ADD UNIQUE KEY `token` (`token`),
  ADD KEY `username_2` (`username`);

--
-- AUTO_INCREMENT for dumped tables
--

--
-- AUTO_INCREMENT for table `devices`
--
ALTER TABLE `devices`
  MODIFY `id` int(10) NOT NULL AUTO_INCREMENT;

--
-- AUTO_INCREMENT for table `security_pool`
--
ALTER TABLE `security_pool`
  MODIFY `id` int(10) NOT NULL AUTO_INCREMENT, AUTO_INCREMENT=3;

--
-- AUTO_INCREMENT for table `users`
--
ALTER TABLE `users`
  MODIFY `id` int(10) NOT NULL AUTO_INCREMENT, AUTO_INCREMENT=21;
COMMIT;

/*!40101 SET CHARACTER_SET_CLIENT=@OLD_CHARACTER_SET_CLIENT */;
/*!40101 SET CHARACTER_SET_RESULTS=@OLD_CHARACTER_SET_RESULTS */;
/*!40101 SET COLLATION_CONNECTION=@OLD_COLLATION_CONNECTION */;
