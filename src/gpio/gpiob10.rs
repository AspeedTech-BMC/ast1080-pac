#[doc = "Register `GPIOB10` reader"]
pub type R = crate::R<Gpiob10Spec>;
#[doc = "Register `GPIOB10` writer"]
pub type W = crate::W<Gpiob10Spec>;
#[doc = "GPIO000 Write Privilege Reset Tolerance\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio000wrPrivilegeRstTolerance {
    #[doc = "0: Write Privilege of GPIO000 is reset by WDT."]
    WritePrivilegeOfGpio000IsResetByWdt = 0,
    #[doc = "1: Write Privilege of GPIO000 is NOT reset by WDT."]
    WritePrivilegeOfGpio000IsNotResetByWdt = 1,
}
impl From<Gpio000wrPrivilegeRstTolerance> for bool {
    #[inline(always)]
    fn from(variant: Gpio000wrPrivilegeRstTolerance) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIO000WrPrivilegeRstTolerance` reader - GPIO000 Write Privilege Reset Tolerance"]
pub type Gpio000wrPrivilegeRstToleranceR = crate::BitReader<Gpio000wrPrivilegeRstTolerance>;
impl Gpio000wrPrivilegeRstToleranceR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio000wrPrivilegeRstTolerance {
        match self.bits {
            false => Gpio000wrPrivilegeRstTolerance::WritePrivilegeOfGpio000IsResetByWdt,
            true => Gpio000wrPrivilegeRstTolerance::WritePrivilegeOfGpio000IsNotResetByWdt,
        }
    }
    #[doc = "Write Privilege of GPIO000 is reset by WDT."]
    #[inline(always)]
    pub fn is_write_privilege_of_gpio000_is_reset_by_wdt(&self) -> bool {
        *self == Gpio000wrPrivilegeRstTolerance::WritePrivilegeOfGpio000IsResetByWdt
    }
    #[doc = "Write Privilege of GPIO000 is NOT reset by WDT."]
    #[inline(always)]
    pub fn is_write_privilege_of_gpio000_is_not_reset_by_wdt(&self) -> bool {
        *self == Gpio000wrPrivilegeRstTolerance::WritePrivilegeOfGpio000IsNotResetByWdt
    }
}
#[doc = "Field `GPIO000WrPrivilegeRstTolerance` writer - GPIO000 Write Privilege Reset Tolerance"]
pub type Gpio000wrPrivilegeRstToleranceW<'a, REG> =
    crate::BitWriter<'a, REG, Gpio000wrPrivilegeRstTolerance>;
impl<'a, REG> Gpio000wrPrivilegeRstToleranceW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Write Privilege of GPIO000 is reset by WDT."]
    #[inline(always)]
    pub fn write_privilege_of_gpio000_is_reset_by_wdt(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio000wrPrivilegeRstTolerance::WritePrivilegeOfGpio000IsResetByWdt)
    }
    #[doc = "Write Privilege of GPIO000 is NOT reset by WDT."]
    #[inline(always)]
    pub fn write_privilege_of_gpio000_is_not_reset_by_wdt(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio000wrPrivilegeRstTolerance::WritePrivilegeOfGpio000IsNotResetByWdt)
    }
}
#[doc = "Field `GPIO001WrPrivilegeRstTolerance` reader - GPIO001 Write Privilege Reset Tolerance"]
pub type Gpio001wrPrivilegeRstToleranceR = crate::BitReader;
#[doc = "Field `GPIO001WrPrivilegeRstTolerance` writer - GPIO001 Write Privilege Reset Tolerance"]
pub type Gpio001wrPrivilegeRstToleranceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIO002WrPrivilegeRstTolerance` reader - GPIO002 Write Privilege Reset Tolerance"]
pub type Gpio002wrPrivilegeRstToleranceR = crate::BitReader;
#[doc = "Field `GPIO002WrPrivilegeRstTolerance` writer - GPIO002 Write Privilege Reset Tolerance"]
pub type Gpio002wrPrivilegeRstToleranceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIO003WrPrivilegeRstTolerance` reader - GPIO003 Write Privilege Reset Tolerance"]
pub type Gpio003wrPrivilegeRstToleranceR = crate::BitReader;
#[doc = "Field `GPIO003WrPrivilegeRstTolerance` writer - GPIO003 Write Privilege Reset Tolerance"]
pub type Gpio003wrPrivilegeRstToleranceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIO004WrPrivilegeRstTolerance` reader - GPIO004 Write Privilege Reset Tolerance"]
pub type Gpio004wrPrivilegeRstToleranceR = crate::BitReader;
#[doc = "Field `GPIO004WrPrivilegeRstTolerance` writer - GPIO004 Write Privilege Reset Tolerance"]
pub type Gpio004wrPrivilegeRstToleranceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIO005WrPrivilegeRstTolerance` reader - GPIO005 Write Privilege Reset Tolerance"]
pub type Gpio005wrPrivilegeRstToleranceR = crate::BitReader;
#[doc = "Field `GPIO005WrPrivilegeRstTolerance` writer - GPIO005 Write Privilege Reset Tolerance"]
pub type Gpio005wrPrivilegeRstToleranceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIO006WrPrivilegeRstTolerance` reader - GPIO006 Write Privilege Reset Tolerance"]
pub type Gpio006wrPrivilegeRstToleranceR = crate::BitReader;
#[doc = "Field `GPIO006WrPrivilegeRstTolerance` writer - GPIO006 Write Privilege Reset Tolerance"]
pub type Gpio006wrPrivilegeRstToleranceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIO007WrPrivilegeRstTolerance` reader - GPIO007 Write Privilege Reset Tolerance"]
pub type Gpio007wrPrivilegeRstToleranceR = crate::BitReader;
#[doc = "Field `GPIO007WrPrivilegeRstTolerance` writer - GPIO007 Write Privilege Reset Tolerance"]
pub type Gpio007wrPrivilegeRstToleranceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIO008WrPrivilegeRstTolerance` reader - GPIO008 Write Privilege Reset Tolerance"]
pub type Gpio008wrPrivilegeRstToleranceR = crate::BitReader;
#[doc = "Field `GPIO008WrPrivilegeRstTolerance` writer - GPIO008 Write Privilege Reset Tolerance"]
pub type Gpio008wrPrivilegeRstToleranceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIO009WrPrivilegeRstTolerance` reader - GPIO009 Write Privilege Reset Tolerance"]
pub type Gpio009wrPrivilegeRstToleranceR = crate::BitReader;
#[doc = "Field `GPIO009WrPrivilegeRstTolerance` writer - GPIO009 Write Privilege Reset Tolerance"]
pub type Gpio009wrPrivilegeRstToleranceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIO010WrPrivilegeRstTolerance` reader - GPIO010 Write Privilege Reset Tolerance"]
pub type Gpio010wrPrivilegeRstToleranceR = crate::BitReader;
#[doc = "Field `GPIO010WrPrivilegeRstTolerance` writer - GPIO010 Write Privilege Reset Tolerance"]
pub type Gpio010wrPrivilegeRstToleranceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIO011WrPrivilegeRstTolerance` reader - GPIO011 Write Privilege Reset Tolerance"]
pub type Gpio011wrPrivilegeRstToleranceR = crate::BitReader;
#[doc = "Field `GPIO011WrPrivilegeRstTolerance` writer - GPIO011 Write Privilege Reset Tolerance"]
pub type Gpio011wrPrivilegeRstToleranceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIO012WrPrivilegeRstTolerance` reader - GPIO012 Write Privilege Reset Tolerance"]
pub type Gpio012wrPrivilegeRstToleranceR = crate::BitReader;
#[doc = "Field `GPIO012WrPrivilegeRstTolerance` writer - GPIO012 Write Privilege Reset Tolerance"]
pub type Gpio012wrPrivilegeRstToleranceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIO013WrPrivilegeRstTolerance` reader - GPIO013 Write Privilege Reset Tolerance"]
pub type Gpio013wrPrivilegeRstToleranceR = crate::BitReader;
#[doc = "Field `GPIO013WrPrivilegeRstTolerance` writer - GPIO013 Write Privilege Reset Tolerance"]
pub type Gpio013wrPrivilegeRstToleranceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIO014WrPrivilegeRstTolerance` reader - GPIO014 Write Privilege Reset Tolerance"]
pub type Gpio014wrPrivilegeRstToleranceR = crate::BitReader;
#[doc = "Field `GPIO014WrPrivilegeRstTolerance` writer - GPIO014 Write Privilege Reset Tolerance"]
pub type Gpio014wrPrivilegeRstToleranceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIO015WrPrivilegeRstTolerance` reader - GPIO015 Write Privilege Reset Tolerance"]
pub type Gpio015wrPrivilegeRstToleranceR = crate::BitReader;
#[doc = "Field `GPIO015WrPrivilegeRstTolerance` writer - GPIO015 Write Privilege Reset Tolerance"]
pub type Gpio015wrPrivilegeRstToleranceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIO016WrPrivilegeRstTolerance` reader - GPIO016 Write Privilege Reset Tolerance"]
pub type Gpio016wrPrivilegeRstToleranceR = crate::BitReader;
#[doc = "Field `GPIO016WrPrivilegeRstTolerance` writer - GPIO016 Write Privilege Reset Tolerance"]
pub type Gpio016wrPrivilegeRstToleranceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIO017WrPrivilegeRstTolerance` reader - GPIO017 Write Privilege Reset Tolerance"]
pub type Gpio017wrPrivilegeRstToleranceR = crate::BitReader;
#[doc = "Field `GPIO017WrPrivilegeRstTolerance` writer - GPIO017 Write Privilege Reset Tolerance"]
pub type Gpio017wrPrivilegeRstToleranceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIO018WrPrivilegeRstTolerance` reader - GPIO018 Write Privilege Reset Tolerance"]
pub type Gpio018wrPrivilegeRstToleranceR = crate::BitReader;
#[doc = "Field `GPIO018WrPrivilegeRstTolerance` writer - GPIO018 Write Privilege Reset Tolerance"]
pub type Gpio018wrPrivilegeRstToleranceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIO019WrPrivilegeRstTolerance` reader - GPIO019 Write Privilege Reset Tolerance"]
pub type Gpio019wrPrivilegeRstToleranceR = crate::BitReader;
#[doc = "Field `GPIO019WrPrivilegeRstTolerance` writer - GPIO019 Write Privilege Reset Tolerance"]
pub type Gpio019wrPrivilegeRstToleranceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIO020WrPrivilegeRstTolerance` reader - GPIO020 Write Privilege Reset Tolerance"]
pub type Gpio020wrPrivilegeRstToleranceR = crate::BitReader;
#[doc = "Field `GPIO020WrPrivilegeRstTolerance` writer - GPIO020 Write Privilege Reset Tolerance"]
pub type Gpio020wrPrivilegeRstToleranceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIO021WrPrivilegeRstTolerance` reader - GPIO021 Write Privilege Reset Tolerance"]
pub type Gpio021wrPrivilegeRstToleranceR = crate::BitReader;
#[doc = "Field `GPIO021WrPrivilegeRstTolerance` writer - GPIO021 Write Privilege Reset Tolerance"]
pub type Gpio021wrPrivilegeRstToleranceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIO022WrPrivilegeRstTolerance` reader - GPIO022 Write Privilege Reset Tolerance"]
pub type Gpio022wrPrivilegeRstToleranceR = crate::BitReader;
#[doc = "Field `GPIO022WrPrivilegeRstTolerance` writer - GPIO022 Write Privilege Reset Tolerance"]
pub type Gpio022wrPrivilegeRstToleranceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIO023WrPrivilegeRstTolerance` reader - GPIO023 Write Privilege Reset Tolerance"]
pub type Gpio023wrPrivilegeRstToleranceR = crate::BitReader;
#[doc = "Field `GPIO023WrPrivilegeRstTolerance` writer - GPIO023 Write Privilege Reset Tolerance"]
pub type Gpio023wrPrivilegeRstToleranceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIO024WrPrivilegeRstTolerance` reader - GPIO024 Write Privilege Reset Tolerance"]
pub type Gpio024wrPrivilegeRstToleranceR = crate::BitReader;
#[doc = "Field `GPIO024WrPrivilegeRstTolerance` writer - GPIO024 Write Privilege Reset Tolerance"]
pub type Gpio024wrPrivilegeRstToleranceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIO025WrPrivilegeRstTolerance` reader - GPIO025 Write Privilege Reset Tolerance"]
pub type Gpio025wrPrivilegeRstToleranceR = crate::BitReader;
#[doc = "Field `GPIO025WrPrivilegeRstTolerance` writer - GPIO025 Write Privilege Reset Tolerance"]
pub type Gpio025wrPrivilegeRstToleranceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIO026WrPrivilegeRstTolerance` reader - GPIO026 Write Privilege Reset Tolerance"]
pub type Gpio026wrPrivilegeRstToleranceR = crate::BitReader;
#[doc = "Field `GPIO026WrPrivilegeRstTolerance` writer - GPIO026 Write Privilege Reset Tolerance"]
pub type Gpio026wrPrivilegeRstToleranceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIO027WrPrivilegeRstTolerance` reader - GPIO027 Write Privilege Reset Tolerance"]
pub type Gpio027wrPrivilegeRstToleranceR = crate::BitReader;
#[doc = "Field `GPIO027WrPrivilegeRstTolerance` writer - GPIO027 Write Privilege Reset Tolerance"]
pub type Gpio027wrPrivilegeRstToleranceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIO028WrPrivilegeRstTolerance` reader - GPIO028 Write Privilege Reset Tolerance"]
pub type Gpio028wrPrivilegeRstToleranceR = crate::BitReader;
#[doc = "Field `GPIO028WrPrivilegeRstTolerance` writer - GPIO028 Write Privilege Reset Tolerance"]
pub type Gpio028wrPrivilegeRstToleranceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIO029WrPrivilegeRstTolerance` reader - GPIO029 Write Privilege Reset Tolerance"]
pub type Gpio029wrPrivilegeRstToleranceR = crate::BitReader;
#[doc = "Field `GPIO029WrPrivilegeRstTolerance` writer - GPIO029 Write Privilege Reset Tolerance"]
pub type Gpio029wrPrivilegeRstToleranceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIO030WrPrivilegeRstTolerance` reader - GPIO030 Write Privilege Reset Tolerance"]
pub type Gpio030wrPrivilegeRstToleranceR = crate::BitReader;
#[doc = "Field `GPIO030WrPrivilegeRstTolerance` writer - GPIO030 Write Privilege Reset Tolerance"]
pub type Gpio030wrPrivilegeRstToleranceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIO031WrPrivilegeRstTolerance` reader - GPIO031 Write Privilege Reset Tolerance"]
pub type Gpio031wrPrivilegeRstToleranceR = crate::BitReader;
#[doc = "Field `GPIO031WrPrivilegeRstTolerance` writer - GPIO031 Write Privilege Reset Tolerance"]
pub type Gpio031wrPrivilegeRstToleranceW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - GPIO000 Write Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio000wr_privilege_rst_tolerance(&self) -> Gpio000wrPrivilegeRstToleranceR {
        Gpio000wrPrivilegeRstToleranceR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - GPIO001 Write Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio001wr_privilege_rst_tolerance(&self) -> Gpio001wrPrivilegeRstToleranceR {
        Gpio001wrPrivilegeRstToleranceR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - GPIO002 Write Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio002wr_privilege_rst_tolerance(&self) -> Gpio002wrPrivilegeRstToleranceR {
        Gpio002wrPrivilegeRstToleranceR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - GPIO003 Write Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio003wr_privilege_rst_tolerance(&self) -> Gpio003wrPrivilegeRstToleranceR {
        Gpio003wrPrivilegeRstToleranceR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - GPIO004 Write Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio004wr_privilege_rst_tolerance(&self) -> Gpio004wrPrivilegeRstToleranceR {
        Gpio004wrPrivilegeRstToleranceR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - GPIO005 Write Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio005wr_privilege_rst_tolerance(&self) -> Gpio005wrPrivilegeRstToleranceR {
        Gpio005wrPrivilegeRstToleranceR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - GPIO006 Write Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio006wr_privilege_rst_tolerance(&self) -> Gpio006wrPrivilegeRstToleranceR {
        Gpio006wrPrivilegeRstToleranceR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - GPIO007 Write Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio007wr_privilege_rst_tolerance(&self) -> Gpio007wrPrivilegeRstToleranceR {
        Gpio007wrPrivilegeRstToleranceR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - GPIO008 Write Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio008wr_privilege_rst_tolerance(&self) -> Gpio008wrPrivilegeRstToleranceR {
        Gpio008wrPrivilegeRstToleranceR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - GPIO009 Write Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio009wr_privilege_rst_tolerance(&self) -> Gpio009wrPrivilegeRstToleranceR {
        Gpio009wrPrivilegeRstToleranceR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - GPIO010 Write Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio010wr_privilege_rst_tolerance(&self) -> Gpio010wrPrivilegeRstToleranceR {
        Gpio010wrPrivilegeRstToleranceR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - GPIO011 Write Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio011wr_privilege_rst_tolerance(&self) -> Gpio011wrPrivilegeRstToleranceR {
        Gpio011wrPrivilegeRstToleranceR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - GPIO012 Write Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio012wr_privilege_rst_tolerance(&self) -> Gpio012wrPrivilegeRstToleranceR {
        Gpio012wrPrivilegeRstToleranceR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - GPIO013 Write Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio013wr_privilege_rst_tolerance(&self) -> Gpio013wrPrivilegeRstToleranceR {
        Gpio013wrPrivilegeRstToleranceR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - GPIO014 Write Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio014wr_privilege_rst_tolerance(&self) -> Gpio014wrPrivilegeRstToleranceR {
        Gpio014wrPrivilegeRstToleranceR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - GPIO015 Write Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio015wr_privilege_rst_tolerance(&self) -> Gpio015wrPrivilegeRstToleranceR {
        Gpio015wrPrivilegeRstToleranceR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - GPIO016 Write Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio016wr_privilege_rst_tolerance(&self) -> Gpio016wrPrivilegeRstToleranceR {
        Gpio016wrPrivilegeRstToleranceR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - GPIO017 Write Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio017wr_privilege_rst_tolerance(&self) -> Gpio017wrPrivilegeRstToleranceR {
        Gpio017wrPrivilegeRstToleranceR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - GPIO018 Write Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio018wr_privilege_rst_tolerance(&self) -> Gpio018wrPrivilegeRstToleranceR {
        Gpio018wrPrivilegeRstToleranceR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - GPIO019 Write Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio019wr_privilege_rst_tolerance(&self) -> Gpio019wrPrivilegeRstToleranceR {
        Gpio019wrPrivilegeRstToleranceR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - GPIO020 Write Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio020wr_privilege_rst_tolerance(&self) -> Gpio020wrPrivilegeRstToleranceR {
        Gpio020wrPrivilegeRstToleranceR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - GPIO021 Write Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio021wr_privilege_rst_tolerance(&self) -> Gpio021wrPrivilegeRstToleranceR {
        Gpio021wrPrivilegeRstToleranceR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - GPIO022 Write Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio022wr_privilege_rst_tolerance(&self) -> Gpio022wrPrivilegeRstToleranceR {
        Gpio022wrPrivilegeRstToleranceR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - GPIO023 Write Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio023wr_privilege_rst_tolerance(&self) -> Gpio023wrPrivilegeRstToleranceR {
        Gpio023wrPrivilegeRstToleranceR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - GPIO024 Write Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio024wr_privilege_rst_tolerance(&self) -> Gpio024wrPrivilegeRstToleranceR {
        Gpio024wrPrivilegeRstToleranceR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - GPIO025 Write Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio025wr_privilege_rst_tolerance(&self) -> Gpio025wrPrivilegeRstToleranceR {
        Gpio025wrPrivilegeRstToleranceR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - GPIO026 Write Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio026wr_privilege_rst_tolerance(&self) -> Gpio026wrPrivilegeRstToleranceR {
        Gpio026wrPrivilegeRstToleranceR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - GPIO027 Write Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio027wr_privilege_rst_tolerance(&self) -> Gpio027wrPrivilegeRstToleranceR {
        Gpio027wrPrivilegeRstToleranceR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - GPIO028 Write Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio028wr_privilege_rst_tolerance(&self) -> Gpio028wrPrivilegeRstToleranceR {
        Gpio028wrPrivilegeRstToleranceR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - GPIO029 Write Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio029wr_privilege_rst_tolerance(&self) -> Gpio029wrPrivilegeRstToleranceR {
        Gpio029wrPrivilegeRstToleranceR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - GPIO030 Write Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio030wr_privilege_rst_tolerance(&self) -> Gpio030wrPrivilegeRstToleranceR {
        Gpio030wrPrivilegeRstToleranceR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - GPIO031 Write Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio031wr_privilege_rst_tolerance(&self) -> Gpio031wrPrivilegeRstToleranceR {
        Gpio031wrPrivilegeRstToleranceR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - GPIO000 Write Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio000wr_privilege_rst_tolerance(
        &mut self,
    ) -> Gpio000wrPrivilegeRstToleranceW<Gpiob10Spec> {
        Gpio000wrPrivilegeRstToleranceW::new(self, 0)
    }
    #[doc = "Bit 1 - GPIO001 Write Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio001wr_privilege_rst_tolerance(
        &mut self,
    ) -> Gpio001wrPrivilegeRstToleranceW<Gpiob10Spec> {
        Gpio001wrPrivilegeRstToleranceW::new(self, 1)
    }
    #[doc = "Bit 2 - GPIO002 Write Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio002wr_privilege_rst_tolerance(
        &mut self,
    ) -> Gpio002wrPrivilegeRstToleranceW<Gpiob10Spec> {
        Gpio002wrPrivilegeRstToleranceW::new(self, 2)
    }
    #[doc = "Bit 3 - GPIO003 Write Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio003wr_privilege_rst_tolerance(
        &mut self,
    ) -> Gpio003wrPrivilegeRstToleranceW<Gpiob10Spec> {
        Gpio003wrPrivilegeRstToleranceW::new(self, 3)
    }
    #[doc = "Bit 4 - GPIO004 Write Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio004wr_privilege_rst_tolerance(
        &mut self,
    ) -> Gpio004wrPrivilegeRstToleranceW<Gpiob10Spec> {
        Gpio004wrPrivilegeRstToleranceW::new(self, 4)
    }
    #[doc = "Bit 5 - GPIO005 Write Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio005wr_privilege_rst_tolerance(
        &mut self,
    ) -> Gpio005wrPrivilegeRstToleranceW<Gpiob10Spec> {
        Gpio005wrPrivilegeRstToleranceW::new(self, 5)
    }
    #[doc = "Bit 6 - GPIO006 Write Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio006wr_privilege_rst_tolerance(
        &mut self,
    ) -> Gpio006wrPrivilegeRstToleranceW<Gpiob10Spec> {
        Gpio006wrPrivilegeRstToleranceW::new(self, 6)
    }
    #[doc = "Bit 7 - GPIO007 Write Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio007wr_privilege_rst_tolerance(
        &mut self,
    ) -> Gpio007wrPrivilegeRstToleranceW<Gpiob10Spec> {
        Gpio007wrPrivilegeRstToleranceW::new(self, 7)
    }
    #[doc = "Bit 8 - GPIO008 Write Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio008wr_privilege_rst_tolerance(
        &mut self,
    ) -> Gpio008wrPrivilegeRstToleranceW<Gpiob10Spec> {
        Gpio008wrPrivilegeRstToleranceW::new(self, 8)
    }
    #[doc = "Bit 9 - GPIO009 Write Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio009wr_privilege_rst_tolerance(
        &mut self,
    ) -> Gpio009wrPrivilegeRstToleranceW<Gpiob10Spec> {
        Gpio009wrPrivilegeRstToleranceW::new(self, 9)
    }
    #[doc = "Bit 10 - GPIO010 Write Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio010wr_privilege_rst_tolerance(
        &mut self,
    ) -> Gpio010wrPrivilegeRstToleranceW<Gpiob10Spec> {
        Gpio010wrPrivilegeRstToleranceW::new(self, 10)
    }
    #[doc = "Bit 11 - GPIO011 Write Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio011wr_privilege_rst_tolerance(
        &mut self,
    ) -> Gpio011wrPrivilegeRstToleranceW<Gpiob10Spec> {
        Gpio011wrPrivilegeRstToleranceW::new(self, 11)
    }
    #[doc = "Bit 12 - GPIO012 Write Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio012wr_privilege_rst_tolerance(
        &mut self,
    ) -> Gpio012wrPrivilegeRstToleranceW<Gpiob10Spec> {
        Gpio012wrPrivilegeRstToleranceW::new(self, 12)
    }
    #[doc = "Bit 13 - GPIO013 Write Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio013wr_privilege_rst_tolerance(
        &mut self,
    ) -> Gpio013wrPrivilegeRstToleranceW<Gpiob10Spec> {
        Gpio013wrPrivilegeRstToleranceW::new(self, 13)
    }
    #[doc = "Bit 14 - GPIO014 Write Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio014wr_privilege_rst_tolerance(
        &mut self,
    ) -> Gpio014wrPrivilegeRstToleranceW<Gpiob10Spec> {
        Gpio014wrPrivilegeRstToleranceW::new(self, 14)
    }
    #[doc = "Bit 15 - GPIO015 Write Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio015wr_privilege_rst_tolerance(
        &mut self,
    ) -> Gpio015wrPrivilegeRstToleranceW<Gpiob10Spec> {
        Gpio015wrPrivilegeRstToleranceW::new(self, 15)
    }
    #[doc = "Bit 16 - GPIO016 Write Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio016wr_privilege_rst_tolerance(
        &mut self,
    ) -> Gpio016wrPrivilegeRstToleranceW<Gpiob10Spec> {
        Gpio016wrPrivilegeRstToleranceW::new(self, 16)
    }
    #[doc = "Bit 17 - GPIO017 Write Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio017wr_privilege_rst_tolerance(
        &mut self,
    ) -> Gpio017wrPrivilegeRstToleranceW<Gpiob10Spec> {
        Gpio017wrPrivilegeRstToleranceW::new(self, 17)
    }
    #[doc = "Bit 18 - GPIO018 Write Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio018wr_privilege_rst_tolerance(
        &mut self,
    ) -> Gpio018wrPrivilegeRstToleranceW<Gpiob10Spec> {
        Gpio018wrPrivilegeRstToleranceW::new(self, 18)
    }
    #[doc = "Bit 19 - GPIO019 Write Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio019wr_privilege_rst_tolerance(
        &mut self,
    ) -> Gpio019wrPrivilegeRstToleranceW<Gpiob10Spec> {
        Gpio019wrPrivilegeRstToleranceW::new(self, 19)
    }
    #[doc = "Bit 20 - GPIO020 Write Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio020wr_privilege_rst_tolerance(
        &mut self,
    ) -> Gpio020wrPrivilegeRstToleranceW<Gpiob10Spec> {
        Gpio020wrPrivilegeRstToleranceW::new(self, 20)
    }
    #[doc = "Bit 21 - GPIO021 Write Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio021wr_privilege_rst_tolerance(
        &mut self,
    ) -> Gpio021wrPrivilegeRstToleranceW<Gpiob10Spec> {
        Gpio021wrPrivilegeRstToleranceW::new(self, 21)
    }
    #[doc = "Bit 22 - GPIO022 Write Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio022wr_privilege_rst_tolerance(
        &mut self,
    ) -> Gpio022wrPrivilegeRstToleranceW<Gpiob10Spec> {
        Gpio022wrPrivilegeRstToleranceW::new(self, 22)
    }
    #[doc = "Bit 23 - GPIO023 Write Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio023wr_privilege_rst_tolerance(
        &mut self,
    ) -> Gpio023wrPrivilegeRstToleranceW<Gpiob10Spec> {
        Gpio023wrPrivilegeRstToleranceW::new(self, 23)
    }
    #[doc = "Bit 24 - GPIO024 Write Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio024wr_privilege_rst_tolerance(
        &mut self,
    ) -> Gpio024wrPrivilegeRstToleranceW<Gpiob10Spec> {
        Gpio024wrPrivilegeRstToleranceW::new(self, 24)
    }
    #[doc = "Bit 25 - GPIO025 Write Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio025wr_privilege_rst_tolerance(
        &mut self,
    ) -> Gpio025wrPrivilegeRstToleranceW<Gpiob10Spec> {
        Gpio025wrPrivilegeRstToleranceW::new(self, 25)
    }
    #[doc = "Bit 26 - GPIO026 Write Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio026wr_privilege_rst_tolerance(
        &mut self,
    ) -> Gpio026wrPrivilegeRstToleranceW<Gpiob10Spec> {
        Gpio026wrPrivilegeRstToleranceW::new(self, 26)
    }
    #[doc = "Bit 27 - GPIO027 Write Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio027wr_privilege_rst_tolerance(
        &mut self,
    ) -> Gpio027wrPrivilegeRstToleranceW<Gpiob10Spec> {
        Gpio027wrPrivilegeRstToleranceW::new(self, 27)
    }
    #[doc = "Bit 28 - GPIO028 Write Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio028wr_privilege_rst_tolerance(
        &mut self,
    ) -> Gpio028wrPrivilegeRstToleranceW<Gpiob10Spec> {
        Gpio028wrPrivilegeRstToleranceW::new(self, 28)
    }
    #[doc = "Bit 29 - GPIO029 Write Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio029wr_privilege_rst_tolerance(
        &mut self,
    ) -> Gpio029wrPrivilegeRstToleranceW<Gpiob10Spec> {
        Gpio029wrPrivilegeRstToleranceW::new(self, 29)
    }
    #[doc = "Bit 30 - GPIO030 Write Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio030wr_privilege_rst_tolerance(
        &mut self,
    ) -> Gpio030wrPrivilegeRstToleranceW<Gpiob10Spec> {
        Gpio030wrPrivilegeRstToleranceW::new(self, 30)
    }
    #[doc = "Bit 31 - GPIO031 Write Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio031wr_privilege_rst_tolerance(
        &mut self,
    ) -> Gpio031wrPrivilegeRstToleranceW<Gpiob10Spec> {
        Gpio031wrPrivilegeRstToleranceW::new(self, 31)
    }
}
#[doc = "Write Privilege Reset Tolerance Register \\#1\n\nYou can [`read`](crate::Reg::read) this register and get [`gpiob10::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpiob10::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gpiob10Spec;
impl crate::RegisterSpec for Gpiob10Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpiob10::R`](R) reader structure"]
impl crate::Readable for Gpiob10Spec {}
#[doc = "`write(|w| ..)` method takes [`gpiob10::W`](W) writer structure"]
impl crate::Writable for Gpiob10Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GPIOB10 to value 0"]
impl crate::Resettable for Gpiob10Spec {}
