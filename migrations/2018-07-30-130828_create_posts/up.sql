SET FOREIGN_KEY_CHECKS=0;

DROP TABLE IF EXISTS `posts`;
CREATE TABLE `posts` (
  `id` int(11) AUTO_INCREMENT,
  `content` text NOT NULL,
  `types` int(11) NOT NULL,
  `agree` int(11) NOT NULL,
  `disagree` int(11) NOT NULL,
  `created_at` TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  PRIMARY KEY (`id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4;
