#[doc = "Register `GPIOC10` reader"]
pub type R = crate::R<Gpioc10Spec>;
#[doc = "Register `GPIOC10` writer"]
pub type W = crate::W<Gpioc10Spec>;
#[doc = "GPIO000 Read Privilege Reset Tolerance\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio000readPrivilegeRstTolerance {
    #[doc = "0: Read Privilege of GPIO000 is reset by WDT."]
    ReadPrivilegeOfGpio000IsResetByWdt = 0,
    #[doc = "1: Read Privilege of GPIO000 is NOT reset by WDT."]
    ReadPrivilegeOfGpio000IsNotResetByWdt = 1,
}
impl From<Gpio000readPrivilegeRstTolerance> for bool {
    #[inline(always)]
    fn from(variant: Gpio000readPrivilegeRstTolerance) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIO000ReadPrivilegeRstTolerance` reader - GPIO000 Read Privilege Reset Tolerance"]
pub type Gpio000readPrivilegeRstToleranceR = crate::BitReader<Gpio000readPrivilegeRstTolerance>;
impl Gpio000readPrivilegeRstToleranceR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio000readPrivilegeRstTolerance {
        match self.bits {
            false => Gpio000readPrivilegeRstTolerance::ReadPrivilegeOfGpio000IsResetByWdt,
            true => Gpio000readPrivilegeRstTolerance::ReadPrivilegeOfGpio000IsNotResetByWdt,
        }
    }
    #[doc = "Read Privilege of GPIO000 is reset by WDT."]
    #[inline(always)]
    pub fn is_read_privilege_of_gpio000_is_reset_by_wdt(&self) -> bool {
        *self == Gpio000readPrivilegeRstTolerance::ReadPrivilegeOfGpio000IsResetByWdt
    }
    #[doc = "Read Privilege of GPIO000 is NOT reset by WDT."]
    #[inline(always)]
    pub fn is_read_privilege_of_gpio000_is_not_reset_by_wdt(&self) -> bool {
        *self == Gpio000readPrivilegeRstTolerance::ReadPrivilegeOfGpio000IsNotResetByWdt
    }
}
#[doc = "Field `GPIO000ReadPrivilegeRstTolerance` writer - GPIO000 Read Privilege Reset Tolerance"]
pub type Gpio000readPrivilegeRstToleranceW<'a, REG> =
    crate::BitWriter<'a, REG, Gpio000readPrivilegeRstTolerance>;
impl<'a, REG> Gpio000readPrivilegeRstToleranceW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Read Privilege of GPIO000 is reset by WDT."]
    #[inline(always)]
    pub fn read_privilege_of_gpio000_is_reset_by_wdt(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio000readPrivilegeRstTolerance::ReadPrivilegeOfGpio000IsResetByWdt)
    }
    #[doc = "Read Privilege of GPIO000 is NOT reset by WDT."]
    #[inline(always)]
    pub fn read_privilege_of_gpio000_is_not_reset_by_wdt(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio000readPrivilegeRstTolerance::ReadPrivilegeOfGpio000IsNotResetByWdt)
    }
}
#[doc = "Field `GPIO001ReadPrivilegeRstTolerance` reader - GPIO001 Read Privilege Reset Tolerance"]
pub type Gpio001readPrivilegeRstToleranceR = crate::BitReader;
#[doc = "Field `GPIO001ReadPrivilegeRstTolerance` writer - GPIO001 Read Privilege Reset Tolerance"]
pub type Gpio001readPrivilegeRstToleranceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIO002ReadPrivilegeRstTolerance` reader - GPIO002 Read Privilege Reset Tolerance"]
pub type Gpio002readPrivilegeRstToleranceR = crate::BitReader;
#[doc = "Field `GPIO002ReadPrivilegeRstTolerance` writer - GPIO002 Read Privilege Reset Tolerance"]
pub type Gpio002readPrivilegeRstToleranceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIO003ReadPrivilegeRstTolerance` reader - GPIO003 Read Privilege Reset Tolerance"]
pub type Gpio003readPrivilegeRstToleranceR = crate::BitReader;
#[doc = "Field `GPIO003ReadPrivilegeRstTolerance` writer - GPIO003 Read Privilege Reset Tolerance"]
pub type Gpio003readPrivilegeRstToleranceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIO004ReadPrivilegeRstTolerance` reader - GPIO004 Read Privilege Reset Tolerance"]
pub type Gpio004readPrivilegeRstToleranceR = crate::BitReader;
#[doc = "Field `GPIO004ReadPrivilegeRstTolerance` writer - GPIO004 Read Privilege Reset Tolerance"]
pub type Gpio004readPrivilegeRstToleranceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIO005ReadPrivilegeRstTolerance` reader - GPIO005 Read Privilege Reset Tolerance"]
pub type Gpio005readPrivilegeRstToleranceR = crate::BitReader;
#[doc = "Field `GPIO005ReadPrivilegeRstTolerance` writer - GPIO005 Read Privilege Reset Tolerance"]
pub type Gpio005readPrivilegeRstToleranceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIO006ReadPrivilegeRstTolerance` reader - GPIO006 Read Privilege Reset Tolerance"]
pub type Gpio006readPrivilegeRstToleranceR = crate::BitReader;
#[doc = "Field `GPIO006ReadPrivilegeRstTolerance` writer - GPIO006 Read Privilege Reset Tolerance"]
pub type Gpio006readPrivilegeRstToleranceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIO007ReadPrivilegeRstTolerance` reader - GPIO007 Read Privilege Reset Tolerance"]
pub type Gpio007readPrivilegeRstToleranceR = crate::BitReader;
#[doc = "Field `GPIO007ReadPrivilegeRstTolerance` writer - GPIO007 Read Privilege Reset Tolerance"]
pub type Gpio007readPrivilegeRstToleranceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIO008ReadPrivilegeRstTolerance` reader - GPIO008 Read Privilege Reset Tolerance"]
pub type Gpio008readPrivilegeRstToleranceR = crate::BitReader;
#[doc = "Field `GPIO008ReadPrivilegeRstTolerance` writer - GPIO008 Read Privilege Reset Tolerance"]
pub type Gpio008readPrivilegeRstToleranceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIO009ReadPrivilegeRstTolerance` reader - GPIO009 Read Privilege Reset Tolerance"]
pub type Gpio009readPrivilegeRstToleranceR = crate::BitReader;
#[doc = "Field `GPIO009ReadPrivilegeRstTolerance` writer - GPIO009 Read Privilege Reset Tolerance"]
pub type Gpio009readPrivilegeRstToleranceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIO010ReadPrivilegeRstTolerance` reader - GPIO010 Read Privilege Reset Tolerance"]
pub type Gpio010readPrivilegeRstToleranceR = crate::BitReader;
#[doc = "Field `GPIO010ReadPrivilegeRstTolerance` writer - GPIO010 Read Privilege Reset Tolerance"]
pub type Gpio010readPrivilegeRstToleranceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIO011ReadPrivilegeRstTolerance` reader - GPIO011 Read Privilege Reset Tolerance"]
pub type Gpio011readPrivilegeRstToleranceR = crate::BitReader;
#[doc = "Field `GPIO011ReadPrivilegeRstTolerance` writer - GPIO011 Read Privilege Reset Tolerance"]
pub type Gpio011readPrivilegeRstToleranceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIO012ReadPrivilegeRstTolerance` reader - GPIO012 Read Privilege Reset Tolerance"]
pub type Gpio012readPrivilegeRstToleranceR = crate::BitReader;
#[doc = "Field `GPIO012ReadPrivilegeRstTolerance` writer - GPIO012 Read Privilege Reset Tolerance"]
pub type Gpio012readPrivilegeRstToleranceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIO013ReadPrivilegeRstTolerance` reader - GPIO013 Read Privilege Reset Tolerance"]
pub type Gpio013readPrivilegeRstToleranceR = crate::BitReader;
#[doc = "Field `GPIO013ReadPrivilegeRstTolerance` writer - GPIO013 Read Privilege Reset Tolerance"]
pub type Gpio013readPrivilegeRstToleranceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIO014ReadPrivilegeRstTolerance` reader - GPIO014 Read Privilege Reset Tolerance"]
pub type Gpio014readPrivilegeRstToleranceR = crate::BitReader;
#[doc = "Field `GPIO014ReadPrivilegeRstTolerance` writer - GPIO014 Read Privilege Reset Tolerance"]
pub type Gpio014readPrivilegeRstToleranceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIO015ReadPrivilegeRstTolerance` reader - GPIO015 Read Privilege Reset Tolerance"]
pub type Gpio015readPrivilegeRstToleranceR = crate::BitReader;
#[doc = "Field `GPIO015ReadPrivilegeRstTolerance` writer - GPIO015 Read Privilege Reset Tolerance"]
pub type Gpio015readPrivilegeRstToleranceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIO016ReadPrivilegeRstTolerance` reader - GPIO016 Read Privilege Reset Tolerance"]
pub type Gpio016readPrivilegeRstToleranceR = crate::BitReader;
#[doc = "Field `GPIO016ReadPrivilegeRstTolerance` writer - GPIO016 Read Privilege Reset Tolerance"]
pub type Gpio016readPrivilegeRstToleranceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIO017ReadPrivilegeRstTolerance` reader - GPIO017 Read Privilege Reset Tolerance"]
pub type Gpio017readPrivilegeRstToleranceR = crate::BitReader;
#[doc = "Field `GPIO017ReadPrivilegeRstTolerance` writer - GPIO017 Read Privilege Reset Tolerance"]
pub type Gpio017readPrivilegeRstToleranceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIO018ReadPrivilegeRstTolerance` reader - GPIO018 Read Privilege Reset Tolerance"]
pub type Gpio018readPrivilegeRstToleranceR = crate::BitReader;
#[doc = "Field `GPIO018ReadPrivilegeRstTolerance` writer - GPIO018 Read Privilege Reset Tolerance"]
pub type Gpio018readPrivilegeRstToleranceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIO019ReadPrivilegeRstTolerance` reader - GPIO019 Read Privilege Reset Tolerance"]
pub type Gpio019readPrivilegeRstToleranceR = crate::BitReader;
#[doc = "Field `GPIO019ReadPrivilegeRstTolerance` writer - GPIO019 Read Privilege Reset Tolerance"]
pub type Gpio019readPrivilegeRstToleranceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIO020ReadPrivilegeRstTolerance` reader - GPIO020 Read Privilege Reset Tolerance"]
pub type Gpio020readPrivilegeRstToleranceR = crate::BitReader;
#[doc = "Field `GPIO020ReadPrivilegeRstTolerance` writer - GPIO020 Read Privilege Reset Tolerance"]
pub type Gpio020readPrivilegeRstToleranceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIO021ReadPrivilegeRstTolerance` reader - GPIO021 Read Privilege Reset Tolerance"]
pub type Gpio021readPrivilegeRstToleranceR = crate::BitReader;
#[doc = "Field `GPIO021ReadPrivilegeRstTolerance` writer - GPIO021 Read Privilege Reset Tolerance"]
pub type Gpio021readPrivilegeRstToleranceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIO022ReadPrivilegeRstTolerance` reader - GPIO022 Read Privilege Reset Tolerance"]
pub type Gpio022readPrivilegeRstToleranceR = crate::BitReader;
#[doc = "Field `GPIO022ReadPrivilegeRstTolerance` writer - GPIO022 Read Privilege Reset Tolerance"]
pub type Gpio022readPrivilegeRstToleranceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIO023ReadPrivilegeRstTolerance` reader - GPIO023 Read Privilege Reset Tolerance"]
pub type Gpio023readPrivilegeRstToleranceR = crate::BitReader;
#[doc = "Field `GPIO023ReadPrivilegeRstTolerance` writer - GPIO023 Read Privilege Reset Tolerance"]
pub type Gpio023readPrivilegeRstToleranceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIO024ReadPrivilegeRstTolerance` reader - GPIO024 Read Privilege Reset Tolerance"]
pub type Gpio024readPrivilegeRstToleranceR = crate::BitReader;
#[doc = "Field `GPIO024ReadPrivilegeRstTolerance` writer - GPIO024 Read Privilege Reset Tolerance"]
pub type Gpio024readPrivilegeRstToleranceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIO025ReadPrivilegeRstTolerance` reader - GPIO025 Read Privilege Reset Tolerance"]
pub type Gpio025readPrivilegeRstToleranceR = crate::BitReader;
#[doc = "Field `GPIO025ReadPrivilegeRstTolerance` writer - GPIO025 Read Privilege Reset Tolerance"]
pub type Gpio025readPrivilegeRstToleranceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIO026ReadPrivilegeRstTolerance` reader - GPIO026 Read Privilege Reset Tolerance"]
pub type Gpio026readPrivilegeRstToleranceR = crate::BitReader;
#[doc = "Field `GPIO026ReadPrivilegeRstTolerance` writer - GPIO026 Read Privilege Reset Tolerance"]
pub type Gpio026readPrivilegeRstToleranceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIO027ReadPrivilegeRstTolerance` reader - GPIO027 Read Privilege Reset Tolerance"]
pub type Gpio027readPrivilegeRstToleranceR = crate::BitReader;
#[doc = "Field `GPIO027ReadPrivilegeRstTolerance` writer - GPIO027 Read Privilege Reset Tolerance"]
pub type Gpio027readPrivilegeRstToleranceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIO028ReadPrivilegeRstTolerance` reader - GPIO028 Read Privilege Reset Tolerance"]
pub type Gpio028readPrivilegeRstToleranceR = crate::BitReader;
#[doc = "Field `GPIO028ReadPrivilegeRstTolerance` writer - GPIO028 Read Privilege Reset Tolerance"]
pub type Gpio028readPrivilegeRstToleranceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIO029ReadPrivilegeRstTolerance` reader - GPIO029 Read Privilege Reset Tolerance"]
pub type Gpio029readPrivilegeRstToleranceR = crate::BitReader;
#[doc = "Field `GPIO029ReadPrivilegeRstTolerance` writer - GPIO029 Read Privilege Reset Tolerance"]
pub type Gpio029readPrivilegeRstToleranceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIO030ReadPrivilegeRstTolerance` reader - GPIO030 Read Privilege Reset Tolerance"]
pub type Gpio030readPrivilegeRstToleranceR = crate::BitReader;
#[doc = "Field `GPIO030ReadPrivilegeRstTolerance` writer - GPIO030 Read Privilege Reset Tolerance"]
pub type Gpio030readPrivilegeRstToleranceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIO031ReadPrivilegeRstTolerance` reader - GPIO031 Read Privilege Reset Tolerance"]
pub type Gpio031readPrivilegeRstToleranceR = crate::BitReader;
#[doc = "Field `GPIO031ReadPrivilegeRstTolerance` writer - GPIO031 Read Privilege Reset Tolerance"]
pub type Gpio031readPrivilegeRstToleranceW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - GPIO000 Read Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio000read_privilege_rst_tolerance(&self) -> Gpio000readPrivilegeRstToleranceR {
        Gpio000readPrivilegeRstToleranceR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - GPIO001 Read Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio001read_privilege_rst_tolerance(&self) -> Gpio001readPrivilegeRstToleranceR {
        Gpio001readPrivilegeRstToleranceR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - GPIO002 Read Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio002read_privilege_rst_tolerance(&self) -> Gpio002readPrivilegeRstToleranceR {
        Gpio002readPrivilegeRstToleranceR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - GPIO003 Read Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio003read_privilege_rst_tolerance(&self) -> Gpio003readPrivilegeRstToleranceR {
        Gpio003readPrivilegeRstToleranceR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - GPIO004 Read Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio004read_privilege_rst_tolerance(&self) -> Gpio004readPrivilegeRstToleranceR {
        Gpio004readPrivilegeRstToleranceR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - GPIO005 Read Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio005read_privilege_rst_tolerance(&self) -> Gpio005readPrivilegeRstToleranceR {
        Gpio005readPrivilegeRstToleranceR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - GPIO006 Read Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio006read_privilege_rst_tolerance(&self) -> Gpio006readPrivilegeRstToleranceR {
        Gpio006readPrivilegeRstToleranceR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - GPIO007 Read Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio007read_privilege_rst_tolerance(&self) -> Gpio007readPrivilegeRstToleranceR {
        Gpio007readPrivilegeRstToleranceR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - GPIO008 Read Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio008read_privilege_rst_tolerance(&self) -> Gpio008readPrivilegeRstToleranceR {
        Gpio008readPrivilegeRstToleranceR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - GPIO009 Read Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio009read_privilege_rst_tolerance(&self) -> Gpio009readPrivilegeRstToleranceR {
        Gpio009readPrivilegeRstToleranceR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - GPIO010 Read Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio010read_privilege_rst_tolerance(&self) -> Gpio010readPrivilegeRstToleranceR {
        Gpio010readPrivilegeRstToleranceR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - GPIO011 Read Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio011read_privilege_rst_tolerance(&self) -> Gpio011readPrivilegeRstToleranceR {
        Gpio011readPrivilegeRstToleranceR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - GPIO012 Read Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio012read_privilege_rst_tolerance(&self) -> Gpio012readPrivilegeRstToleranceR {
        Gpio012readPrivilegeRstToleranceR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - GPIO013 Read Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio013read_privilege_rst_tolerance(&self) -> Gpio013readPrivilegeRstToleranceR {
        Gpio013readPrivilegeRstToleranceR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - GPIO014 Read Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio014read_privilege_rst_tolerance(&self) -> Gpio014readPrivilegeRstToleranceR {
        Gpio014readPrivilegeRstToleranceR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - GPIO015 Read Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio015read_privilege_rst_tolerance(&self) -> Gpio015readPrivilegeRstToleranceR {
        Gpio015readPrivilegeRstToleranceR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - GPIO016 Read Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio016read_privilege_rst_tolerance(&self) -> Gpio016readPrivilegeRstToleranceR {
        Gpio016readPrivilegeRstToleranceR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - GPIO017 Read Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio017read_privilege_rst_tolerance(&self) -> Gpio017readPrivilegeRstToleranceR {
        Gpio017readPrivilegeRstToleranceR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - GPIO018 Read Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio018read_privilege_rst_tolerance(&self) -> Gpio018readPrivilegeRstToleranceR {
        Gpio018readPrivilegeRstToleranceR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - GPIO019 Read Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio019read_privilege_rst_tolerance(&self) -> Gpio019readPrivilegeRstToleranceR {
        Gpio019readPrivilegeRstToleranceR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - GPIO020 Read Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio020read_privilege_rst_tolerance(&self) -> Gpio020readPrivilegeRstToleranceR {
        Gpio020readPrivilegeRstToleranceR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - GPIO021 Read Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio021read_privilege_rst_tolerance(&self) -> Gpio021readPrivilegeRstToleranceR {
        Gpio021readPrivilegeRstToleranceR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - GPIO022 Read Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio022read_privilege_rst_tolerance(&self) -> Gpio022readPrivilegeRstToleranceR {
        Gpio022readPrivilegeRstToleranceR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - GPIO023 Read Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio023read_privilege_rst_tolerance(&self) -> Gpio023readPrivilegeRstToleranceR {
        Gpio023readPrivilegeRstToleranceR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - GPIO024 Read Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio024read_privilege_rst_tolerance(&self) -> Gpio024readPrivilegeRstToleranceR {
        Gpio024readPrivilegeRstToleranceR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - GPIO025 Read Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio025read_privilege_rst_tolerance(&self) -> Gpio025readPrivilegeRstToleranceR {
        Gpio025readPrivilegeRstToleranceR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - GPIO026 Read Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio026read_privilege_rst_tolerance(&self) -> Gpio026readPrivilegeRstToleranceR {
        Gpio026readPrivilegeRstToleranceR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - GPIO027 Read Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio027read_privilege_rst_tolerance(&self) -> Gpio027readPrivilegeRstToleranceR {
        Gpio027readPrivilegeRstToleranceR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - GPIO028 Read Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio028read_privilege_rst_tolerance(&self) -> Gpio028readPrivilegeRstToleranceR {
        Gpio028readPrivilegeRstToleranceR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - GPIO029 Read Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio029read_privilege_rst_tolerance(&self) -> Gpio029readPrivilegeRstToleranceR {
        Gpio029readPrivilegeRstToleranceR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - GPIO030 Read Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio030read_privilege_rst_tolerance(&self) -> Gpio030readPrivilegeRstToleranceR {
        Gpio030readPrivilegeRstToleranceR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - GPIO031 Read Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio031read_privilege_rst_tolerance(&self) -> Gpio031readPrivilegeRstToleranceR {
        Gpio031readPrivilegeRstToleranceR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - GPIO000 Read Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio000read_privilege_rst_tolerance(
        &mut self,
    ) -> Gpio000readPrivilegeRstToleranceW<Gpioc10Spec> {
        Gpio000readPrivilegeRstToleranceW::new(self, 0)
    }
    #[doc = "Bit 1 - GPIO001 Read Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio001read_privilege_rst_tolerance(
        &mut self,
    ) -> Gpio001readPrivilegeRstToleranceW<Gpioc10Spec> {
        Gpio001readPrivilegeRstToleranceW::new(self, 1)
    }
    #[doc = "Bit 2 - GPIO002 Read Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio002read_privilege_rst_tolerance(
        &mut self,
    ) -> Gpio002readPrivilegeRstToleranceW<Gpioc10Spec> {
        Gpio002readPrivilegeRstToleranceW::new(self, 2)
    }
    #[doc = "Bit 3 - GPIO003 Read Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio003read_privilege_rst_tolerance(
        &mut self,
    ) -> Gpio003readPrivilegeRstToleranceW<Gpioc10Spec> {
        Gpio003readPrivilegeRstToleranceW::new(self, 3)
    }
    #[doc = "Bit 4 - GPIO004 Read Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio004read_privilege_rst_tolerance(
        &mut self,
    ) -> Gpio004readPrivilegeRstToleranceW<Gpioc10Spec> {
        Gpio004readPrivilegeRstToleranceW::new(self, 4)
    }
    #[doc = "Bit 5 - GPIO005 Read Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio005read_privilege_rst_tolerance(
        &mut self,
    ) -> Gpio005readPrivilegeRstToleranceW<Gpioc10Spec> {
        Gpio005readPrivilegeRstToleranceW::new(self, 5)
    }
    #[doc = "Bit 6 - GPIO006 Read Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio006read_privilege_rst_tolerance(
        &mut self,
    ) -> Gpio006readPrivilegeRstToleranceW<Gpioc10Spec> {
        Gpio006readPrivilegeRstToleranceW::new(self, 6)
    }
    #[doc = "Bit 7 - GPIO007 Read Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio007read_privilege_rst_tolerance(
        &mut self,
    ) -> Gpio007readPrivilegeRstToleranceW<Gpioc10Spec> {
        Gpio007readPrivilegeRstToleranceW::new(self, 7)
    }
    #[doc = "Bit 8 - GPIO008 Read Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio008read_privilege_rst_tolerance(
        &mut self,
    ) -> Gpio008readPrivilegeRstToleranceW<Gpioc10Spec> {
        Gpio008readPrivilegeRstToleranceW::new(self, 8)
    }
    #[doc = "Bit 9 - GPIO009 Read Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio009read_privilege_rst_tolerance(
        &mut self,
    ) -> Gpio009readPrivilegeRstToleranceW<Gpioc10Spec> {
        Gpio009readPrivilegeRstToleranceW::new(self, 9)
    }
    #[doc = "Bit 10 - GPIO010 Read Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio010read_privilege_rst_tolerance(
        &mut self,
    ) -> Gpio010readPrivilegeRstToleranceW<Gpioc10Spec> {
        Gpio010readPrivilegeRstToleranceW::new(self, 10)
    }
    #[doc = "Bit 11 - GPIO011 Read Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio011read_privilege_rst_tolerance(
        &mut self,
    ) -> Gpio011readPrivilegeRstToleranceW<Gpioc10Spec> {
        Gpio011readPrivilegeRstToleranceW::new(self, 11)
    }
    #[doc = "Bit 12 - GPIO012 Read Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio012read_privilege_rst_tolerance(
        &mut self,
    ) -> Gpio012readPrivilegeRstToleranceW<Gpioc10Spec> {
        Gpio012readPrivilegeRstToleranceW::new(self, 12)
    }
    #[doc = "Bit 13 - GPIO013 Read Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio013read_privilege_rst_tolerance(
        &mut self,
    ) -> Gpio013readPrivilegeRstToleranceW<Gpioc10Spec> {
        Gpio013readPrivilegeRstToleranceW::new(self, 13)
    }
    #[doc = "Bit 14 - GPIO014 Read Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio014read_privilege_rst_tolerance(
        &mut self,
    ) -> Gpio014readPrivilegeRstToleranceW<Gpioc10Spec> {
        Gpio014readPrivilegeRstToleranceW::new(self, 14)
    }
    #[doc = "Bit 15 - GPIO015 Read Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio015read_privilege_rst_tolerance(
        &mut self,
    ) -> Gpio015readPrivilegeRstToleranceW<Gpioc10Spec> {
        Gpio015readPrivilegeRstToleranceW::new(self, 15)
    }
    #[doc = "Bit 16 - GPIO016 Read Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio016read_privilege_rst_tolerance(
        &mut self,
    ) -> Gpio016readPrivilegeRstToleranceW<Gpioc10Spec> {
        Gpio016readPrivilegeRstToleranceW::new(self, 16)
    }
    #[doc = "Bit 17 - GPIO017 Read Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio017read_privilege_rst_tolerance(
        &mut self,
    ) -> Gpio017readPrivilegeRstToleranceW<Gpioc10Spec> {
        Gpio017readPrivilegeRstToleranceW::new(self, 17)
    }
    #[doc = "Bit 18 - GPIO018 Read Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio018read_privilege_rst_tolerance(
        &mut self,
    ) -> Gpio018readPrivilegeRstToleranceW<Gpioc10Spec> {
        Gpio018readPrivilegeRstToleranceW::new(self, 18)
    }
    #[doc = "Bit 19 - GPIO019 Read Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio019read_privilege_rst_tolerance(
        &mut self,
    ) -> Gpio019readPrivilegeRstToleranceW<Gpioc10Spec> {
        Gpio019readPrivilegeRstToleranceW::new(self, 19)
    }
    #[doc = "Bit 20 - GPIO020 Read Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio020read_privilege_rst_tolerance(
        &mut self,
    ) -> Gpio020readPrivilegeRstToleranceW<Gpioc10Spec> {
        Gpio020readPrivilegeRstToleranceW::new(self, 20)
    }
    #[doc = "Bit 21 - GPIO021 Read Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio021read_privilege_rst_tolerance(
        &mut self,
    ) -> Gpio021readPrivilegeRstToleranceW<Gpioc10Spec> {
        Gpio021readPrivilegeRstToleranceW::new(self, 21)
    }
    #[doc = "Bit 22 - GPIO022 Read Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio022read_privilege_rst_tolerance(
        &mut self,
    ) -> Gpio022readPrivilegeRstToleranceW<Gpioc10Spec> {
        Gpio022readPrivilegeRstToleranceW::new(self, 22)
    }
    #[doc = "Bit 23 - GPIO023 Read Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio023read_privilege_rst_tolerance(
        &mut self,
    ) -> Gpio023readPrivilegeRstToleranceW<Gpioc10Spec> {
        Gpio023readPrivilegeRstToleranceW::new(self, 23)
    }
    #[doc = "Bit 24 - GPIO024 Read Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio024read_privilege_rst_tolerance(
        &mut self,
    ) -> Gpio024readPrivilegeRstToleranceW<Gpioc10Spec> {
        Gpio024readPrivilegeRstToleranceW::new(self, 24)
    }
    #[doc = "Bit 25 - GPIO025 Read Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio025read_privilege_rst_tolerance(
        &mut self,
    ) -> Gpio025readPrivilegeRstToleranceW<Gpioc10Spec> {
        Gpio025readPrivilegeRstToleranceW::new(self, 25)
    }
    #[doc = "Bit 26 - GPIO026 Read Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio026read_privilege_rst_tolerance(
        &mut self,
    ) -> Gpio026readPrivilegeRstToleranceW<Gpioc10Spec> {
        Gpio026readPrivilegeRstToleranceW::new(self, 26)
    }
    #[doc = "Bit 27 - GPIO027 Read Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio027read_privilege_rst_tolerance(
        &mut self,
    ) -> Gpio027readPrivilegeRstToleranceW<Gpioc10Spec> {
        Gpio027readPrivilegeRstToleranceW::new(self, 27)
    }
    #[doc = "Bit 28 - GPIO028 Read Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio028read_privilege_rst_tolerance(
        &mut self,
    ) -> Gpio028readPrivilegeRstToleranceW<Gpioc10Spec> {
        Gpio028readPrivilegeRstToleranceW::new(self, 28)
    }
    #[doc = "Bit 29 - GPIO029 Read Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio029read_privilege_rst_tolerance(
        &mut self,
    ) -> Gpio029readPrivilegeRstToleranceW<Gpioc10Spec> {
        Gpio029readPrivilegeRstToleranceW::new(self, 29)
    }
    #[doc = "Bit 30 - GPIO030 Read Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio030read_privilege_rst_tolerance(
        &mut self,
    ) -> Gpio030readPrivilegeRstToleranceW<Gpioc10Spec> {
        Gpio030readPrivilegeRstToleranceW::new(self, 30)
    }
    #[doc = "Bit 31 - GPIO031 Read Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio031read_privilege_rst_tolerance(
        &mut self,
    ) -> Gpio031readPrivilegeRstToleranceW<Gpioc10Spec> {
        Gpio031readPrivilegeRstToleranceW::new(self, 31)
    }
}
#[doc = "Read Privilege Reset Tolerance Register \\#1\n\nYou can [`read`](crate::Reg::read) this register and get [`gpioc10::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpioc10::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gpioc10Spec;
impl crate::RegisterSpec for Gpioc10Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpioc10::R`](R) reader structure"]
impl crate::Readable for Gpioc10Spec {}
#[doc = "`write(|w| ..)` method takes [`gpioc10::W`](W) writer structure"]
impl crate::Writable for Gpioc10Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GPIOC10 to value 0"]
impl crate::Resettable for Gpioc10Spec {}
