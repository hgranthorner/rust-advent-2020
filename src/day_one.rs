use std::collections::HashSet;

pub fn solve_first(s: &str) -> Option<usize> {
    const TARGET: usize = 2020;
    let nums: HashSet<usize> = s.split('\n').map(|s| s.parse().unwrap()).collect();

    for num in &nums {
        let other = TARGET - num;
        if nums.contains(&other) {
            return Some(num * other);
        }
    }

    None
}

pub fn solve_second(s: &str) -> usize {
    const TARGET: usize = 2020;
    let nums: Vec<usize> = s.split('\n').map(|s| s.parse().unwrap()).collect();
    for i in 0..(nums.len() - 2) {
        let num1 = nums[i];
        for j in (i + 1)..(nums.len() - 1) {
            let num2 = nums[j];
            for num3 in nums.get(j + 1..).unwrap() {
                if num1 + num2 + num3 == TARGET {
                    return num1 * num2 * num3;
                }
            }
        }
    }

    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve_first() {
        let input = "1721
979
366
299
675
1456";
        if let Some(num) = solve_first(input) {
            assert_eq!(num, 514579)
        } else {
            panic!("Failed to solve first test!")
        }
    }

    #[test]
    fn test_solve_second() {
        let input = "1721
979
366
299
675
1456";
        let num = solve_second(input);
        assert_eq!(num, 241861950)
    }

    #[test]
    fn get_solution_one() {
        if let Some(num) = solve_first(INPUT) {
            assert_eq!(num, 1015476)
        } else {
            panic!("Failed to solve first!")
        }
    }

    #[test]
    fn get_solution_two() {
        assert_eq!(solve_second(INPUT), 200878544)
    }

    const INPUT: &str = "1780
1693
1830
1756
1858
1868
1968
1809
1996
1962
1800
1974
1805
1795
170
1684
1659
1713
1848
1749
1717
1734
956
1782
1834
1785
1786
1994
1652
1669
1812
1954
1984
1665
1987
1562
2004
2010
1551
961
1854
2005
1883
1965
475
1776
1791
262
1912
1227
1486
1989
1857
825
1683
1991
1875
1982
1654
1767
1673
1973
1886
1731
1745
1770
1995
1721
1662
1679
1783
1999
1889
1746
1902
2003
1698
1794
1798
1951
1953
2007
1899
1658
1705
62
1819
1708
1666
2006
1763
1732
1613
1841
1747
1489
1845
2008
1885
2002
1735
1656
1771
1950
1704
1737
1748
1759
1802
2000
1955
1738
1761
1765
1853
1900
1709
1979
1911
1775
1813
1949
1966
1774
1977
1757
1992
2009
1956
1840
1988
1985
1993
1718
1976
1078
1997
1897
1792
1790
1801
1871
1727
1700
1485
942
1686
1859
1676
802
1952
1998
1961
1844
1808
1703
1980
1766
1963
1849
1670
1716
1957
1660
1816
1762
1829
526
359
2001
1874
1778
1873
1511
1810
1699
1970
1690
1978
1892
1691
1781
1777
1975
1967
1694
1969
1959
1910
1826
1672
1655
1839
1986
1872
1983
1981
1972
1772
1760";
}
