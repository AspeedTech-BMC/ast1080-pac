#[doc = "Register `GPIOC1C` reader"]
pub type R = crate::R<Gpioc1cSpec>;
#[doc = "Register `GPIOC1C` writer"]
pub type W = crate::W<Gpioc1cSpec>;
#[doc = "GPIO096 Read Privilege Reset Tolerance\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio096readPrivilegeRstTolerance {
    #[doc = "0: Read Privilege of GPIO096 is reset by WDT."]
    ReadPrivilegeOfGpio096IsResetByWdt = 0,
    #[doc = "1: Read Privilege of GPIO096 is NOT reset by WDT."]
    ReadPrivilegeOfGpio096IsNotResetByWdt = 1,
}
impl From<Gpio096readPrivilegeRstTolerance> for bool {
    #[inline(always)]
    fn from(variant: Gpio096readPrivilegeRstTolerance) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIO096ReadPrivilegeRstTolerance` reader - GPIO096 Read Privilege Reset Tolerance"]
pub type Gpio096readPrivilegeRstToleranceR = crate::BitReader<Gpio096readPrivilegeRstTolerance>;
impl Gpio096readPrivilegeRstToleranceR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio096readPrivilegeRstTolerance {
        match self.bits {
            false => Gpio096readPrivilegeRstTolerance::ReadPrivilegeOfGpio096IsResetByWdt,
            true => Gpio096readPrivilegeRstTolerance::ReadPrivilegeOfGpio096IsNotResetByWdt,
        }
    }
    #[doc = "Read Privilege of GPIO096 is reset by WDT."]
    #[inline(always)]
    pub fn is_read_privilege_of_gpio096_is_reset_by_wdt(&self) -> bool {
        *self == Gpio096readPrivilegeRstTolerance::ReadPrivilegeOfGpio096IsResetByWdt
    }
    #[doc = "Read Privilege of GPIO096 is NOT reset by WDT."]
    #[inline(always)]
    pub fn is_read_privilege_of_gpio096_is_not_reset_by_wdt(&self) -> bool {
        *self == Gpio096readPrivilegeRstTolerance::ReadPrivilegeOfGpio096IsNotResetByWdt
    }
}
#[doc = "Field `GPIO096ReadPrivilegeRstTolerance` writer - GPIO096 Read Privilege Reset Tolerance"]
pub type Gpio096readPrivilegeRstToleranceW<'a, REG> =
    crate::BitWriter<'a, REG, Gpio096readPrivilegeRstTolerance>;
impl<'a, REG> Gpio096readPrivilegeRstToleranceW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Read Privilege of GPIO096 is reset by WDT."]
    #[inline(always)]
    pub fn read_privilege_of_gpio096_is_reset_by_wdt(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio096readPrivilegeRstTolerance::ReadPrivilegeOfGpio096IsResetByWdt)
    }
    #[doc = "Read Privilege of GPIO096 is NOT reset by WDT."]
    #[inline(always)]
    pub fn read_privilege_of_gpio096_is_not_reset_by_wdt(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio096readPrivilegeRstTolerance::ReadPrivilegeOfGpio096IsNotResetByWdt)
    }
}
#[doc = "Field `GPIO097ReadPrivilegeRstTolerance` reader - GPIO097 Read Privilege Reset Tolerance"]
pub type Gpio097readPrivilegeRstToleranceR = crate::BitReader;
#[doc = "Field `GPIO097ReadPrivilegeRstTolerance` writer - GPIO097 Read Privilege Reset Tolerance"]
pub type Gpio097readPrivilegeRstToleranceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIO098ReadPrivilegeRstTolerance` reader - GPIO098 Read Privilege Reset Tolerance"]
pub type Gpio098readPrivilegeRstToleranceR = crate::BitReader;
#[doc = "Field `GPIO098ReadPrivilegeRstTolerance` writer - GPIO098 Read Privilege Reset Tolerance"]
pub type Gpio098readPrivilegeRstToleranceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIO099ReadPrivilegeRstTolerance` reader - GPIO099 Read Privilege Reset Tolerance"]
pub type Gpio099readPrivilegeRstToleranceR = crate::BitReader;
#[doc = "Field `GPIO099ReadPrivilegeRstTolerance` writer - GPIO099 Read Privilege Reset Tolerance"]
pub type Gpio099readPrivilegeRstToleranceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIO100ReadPrivilegeRstTolerance` reader - GPIO100 Read Privilege Reset Tolerance"]
pub type Gpio100readPrivilegeRstToleranceR = crate::BitReader;
#[doc = "Field `GPIO100ReadPrivilegeRstTolerance` writer - GPIO100 Read Privilege Reset Tolerance"]
pub type Gpio100readPrivilegeRstToleranceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIO101ReadPrivilegeRstTolerance` reader - GPIO101 Read Privilege Reset Tolerance"]
pub type Gpio101readPrivilegeRstToleranceR = crate::BitReader;
#[doc = "Field `GPIO101ReadPrivilegeRstTolerance` writer - GPIO101 Read Privilege Reset Tolerance"]
pub type Gpio101readPrivilegeRstToleranceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIO102ReadPrivilegeRstTolerance` reader - GPIO102 Read Privilege Reset Tolerance"]
pub type Gpio102readPrivilegeRstToleranceR = crate::BitReader;
#[doc = "Field `GPIO102ReadPrivilegeRstTolerance` writer - GPIO102 Read Privilege Reset Tolerance"]
pub type Gpio102readPrivilegeRstToleranceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIO103ReadPrivilegeRstTolerance` reader - GPIO103 Read Privilege Reset Tolerance"]
pub type Gpio103readPrivilegeRstToleranceR = crate::BitReader;
#[doc = "Field `GPIO103ReadPrivilegeRstTolerance` writer - GPIO103 Read Privilege Reset Tolerance"]
pub type Gpio103readPrivilegeRstToleranceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIO104ReadPrivilegeRstTolerance` reader - GPIO104 Read Privilege Reset Tolerance"]
pub type Gpio104readPrivilegeRstToleranceR = crate::BitReader;
#[doc = "Field `GPIO104ReadPrivilegeRstTolerance` writer - GPIO104 Read Privilege Reset Tolerance"]
pub type Gpio104readPrivilegeRstToleranceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIO105ReadPrivilegeRstTolerance` reader - GPIO105 Read Privilege Reset Tolerance"]
pub type Gpio105readPrivilegeRstToleranceR = crate::BitReader;
#[doc = "Field `GPIO105ReadPrivilegeRstTolerance` writer - GPIO105 Read Privilege Reset Tolerance"]
pub type Gpio105readPrivilegeRstToleranceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIO106ReadPrivilegeRstTolerance` reader - GPIO106 Read Privilege Reset Tolerance"]
pub type Gpio106readPrivilegeRstToleranceR = crate::BitReader;
#[doc = "Field `GPIO106ReadPrivilegeRstTolerance` writer - GPIO106 Read Privilege Reset Tolerance"]
pub type Gpio106readPrivilegeRstToleranceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIO107ReadPrivilegeRstTolerance` reader - GPIO107 Read Privilege Reset Tolerance"]
pub type Gpio107readPrivilegeRstToleranceR = crate::BitReader;
#[doc = "Field `GPIO107ReadPrivilegeRstTolerance` writer - GPIO107 Read Privilege Reset Tolerance"]
pub type Gpio107readPrivilegeRstToleranceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIO108ReadPrivilegeRstTolerance` reader - GPIO108 Read Privilege Reset Tolerance"]
pub type Gpio108readPrivilegeRstToleranceR = crate::BitReader;
#[doc = "Field `GPIO108ReadPrivilegeRstTolerance` writer - GPIO108 Read Privilege Reset Tolerance"]
pub type Gpio108readPrivilegeRstToleranceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIO109ReadPrivilegeRstTolerance` reader - GPIO109 Read Privilege Reset Tolerance"]
pub type Gpio109readPrivilegeRstToleranceR = crate::BitReader;
#[doc = "Field `GPIO109ReadPrivilegeRstTolerance` writer - GPIO109 Read Privilege Reset Tolerance"]
pub type Gpio109readPrivilegeRstToleranceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIO110ReadPrivilegeRstTolerance` reader - GPIO110 Read Privilege Reset Tolerance"]
pub type Gpio110readPrivilegeRstToleranceR = crate::BitReader;
#[doc = "Field `GPIO110ReadPrivilegeRstTolerance` writer - GPIO110 Read Privilege Reset Tolerance"]
pub type Gpio110readPrivilegeRstToleranceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIO111ReadPrivilegeRstTolerance` reader - GPIO111 Read Privilege Reset Tolerance"]
pub type Gpio111readPrivilegeRstToleranceR = crate::BitReader;
#[doc = "Field `GPIO111ReadPrivilegeRstTolerance` writer - GPIO111 Read Privilege Reset Tolerance"]
pub type Gpio111readPrivilegeRstToleranceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIO112ReadPrivilegeRstTolerance` reader - GPIO112 Read Privilege Reset Tolerance"]
pub type Gpio112readPrivilegeRstToleranceR = crate::BitReader;
#[doc = "Field `GPIO112ReadPrivilegeRstTolerance` writer - GPIO112 Read Privilege Reset Tolerance"]
pub type Gpio112readPrivilegeRstToleranceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIO113ReadPrivilegeRstTolerance` reader - GPIO113 Read Privilege Reset Tolerance"]
pub type Gpio113readPrivilegeRstToleranceR = crate::BitReader;
#[doc = "Field `GPIO113ReadPrivilegeRstTolerance` writer - GPIO113 Read Privilege Reset Tolerance"]
pub type Gpio113readPrivilegeRstToleranceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIO114ReadPrivilegeRstTolerance` reader - GPIO114 Read Privilege Reset Tolerance"]
pub type Gpio114readPrivilegeRstToleranceR = crate::BitReader;
#[doc = "Field `GPIO114ReadPrivilegeRstTolerance` writer - GPIO114 Read Privilege Reset Tolerance"]
pub type Gpio114readPrivilegeRstToleranceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIO115ReadPrivilegeRstTolerance` reader - GPIO115 Read Privilege Reset Tolerance"]
pub type Gpio115readPrivilegeRstToleranceR = crate::BitReader;
#[doc = "Field `GPIO115ReadPrivilegeRstTolerance` writer - GPIO115 Read Privilege Reset Tolerance"]
pub type Gpio115readPrivilegeRstToleranceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIO116ReadPrivilegeRstTolerance` reader - GPIO116 Read Privilege Reset Tolerance"]
pub type Gpio116readPrivilegeRstToleranceR = crate::BitReader;
#[doc = "Field `GPIO116ReadPrivilegeRstTolerance` writer - GPIO116 Read Privilege Reset Tolerance"]
pub type Gpio116readPrivilegeRstToleranceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIO117ReadPrivilegeRstTolerance` reader - GPIO117 Read Privilege Reset Tolerance"]
pub type Gpio117readPrivilegeRstToleranceR = crate::BitReader;
#[doc = "Field `GPIO117ReadPrivilegeRstTolerance` writer - GPIO117 Read Privilege Reset Tolerance"]
pub type Gpio117readPrivilegeRstToleranceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIO118ReadPrivilegeRstTolerance` reader - GPIO118 Read Privilege Reset Tolerance"]
pub type Gpio118readPrivilegeRstToleranceR = crate::BitReader;
#[doc = "Field `GPIO118ReadPrivilegeRstTolerance` writer - GPIO118 Read Privilege Reset Tolerance"]
pub type Gpio118readPrivilegeRstToleranceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIO119ReadPrivilegeRstTolerance` reader - GPIO119 Read Privilege Reset Tolerance"]
pub type Gpio119readPrivilegeRstToleranceR = crate::BitReader;
#[doc = "Field `GPIO119ReadPrivilegeRstTolerance` writer - GPIO119 Read Privilege Reset Tolerance"]
pub type Gpio119readPrivilegeRstToleranceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIO120ReadPrivilegeRstTolerance` reader - GPIO120 Read Privilege Reset Tolerance"]
pub type Gpio120readPrivilegeRstToleranceR = crate::BitReader;
#[doc = "Field `GPIO120ReadPrivilegeRstTolerance` writer - GPIO120 Read Privilege Reset Tolerance"]
pub type Gpio120readPrivilegeRstToleranceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIO121ReadPrivilegeRstTolerance` reader - GPIO121 Read Privilege Reset Tolerance"]
pub type Gpio121readPrivilegeRstToleranceR = crate::BitReader;
#[doc = "Field `GPIO121ReadPrivilegeRstTolerance` writer - GPIO121 Read Privilege Reset Tolerance"]
pub type Gpio121readPrivilegeRstToleranceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIO122ReadPrivilegeRstTolerance` reader - GPIO122 Read Privilege Reset Tolerance"]
pub type Gpio122readPrivilegeRstToleranceR = crate::BitReader;
#[doc = "Field `GPIO122ReadPrivilegeRstTolerance` writer - GPIO122 Read Privilege Reset Tolerance"]
pub type Gpio122readPrivilegeRstToleranceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIO123ReadPrivilegeRstTolerance` reader - GPIO123 Read Privilege Reset Tolerance"]
pub type Gpio123readPrivilegeRstToleranceR = crate::BitReader;
#[doc = "Field `GPIO123ReadPrivilegeRstTolerance` writer - GPIO123 Read Privilege Reset Tolerance"]
pub type Gpio123readPrivilegeRstToleranceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIO124ReadPrivilegeRstTolerance` reader - GPIO124 Read Privilege Reset Tolerance"]
pub type Gpio124readPrivilegeRstToleranceR = crate::BitReader;
#[doc = "Field `GPIO124ReadPrivilegeRstTolerance` writer - GPIO124 Read Privilege Reset Tolerance"]
pub type Gpio124readPrivilegeRstToleranceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIO125ReadPrivilegeRstTolerance` reader - GPIO125 Read Privilege Reset Tolerance"]
pub type Gpio125readPrivilegeRstToleranceR = crate::BitReader;
#[doc = "Field `GPIO125ReadPrivilegeRstTolerance` writer - GPIO125 Read Privilege Reset Tolerance"]
pub type Gpio125readPrivilegeRstToleranceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIO126ReadPrivilegeRstTolerance` reader - GPIO126 Read Privilege Reset Tolerance"]
pub type Gpio126readPrivilegeRstToleranceR = crate::BitReader;
#[doc = "Field `GPIO126ReadPrivilegeRstTolerance` writer - GPIO126 Read Privilege Reset Tolerance"]
pub type Gpio126readPrivilegeRstToleranceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIO127ReadPrivilegeRstTolerance` reader - GPIO127 Read Privilege Reset Tolerance"]
pub type Gpio127readPrivilegeRstToleranceR = crate::BitReader;
#[doc = "Field `GPIO127ReadPrivilegeRstTolerance` writer - GPIO127 Read Privilege Reset Tolerance"]
pub type Gpio127readPrivilegeRstToleranceW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - GPIO096 Read Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio096read_privilege_rst_tolerance(&self) -> Gpio096readPrivilegeRstToleranceR {
        Gpio096readPrivilegeRstToleranceR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - GPIO097 Read Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio097read_privilege_rst_tolerance(&self) -> Gpio097readPrivilegeRstToleranceR {
        Gpio097readPrivilegeRstToleranceR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - GPIO098 Read Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio098read_privilege_rst_tolerance(&self) -> Gpio098readPrivilegeRstToleranceR {
        Gpio098readPrivilegeRstToleranceR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - GPIO099 Read Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio099read_privilege_rst_tolerance(&self) -> Gpio099readPrivilegeRstToleranceR {
        Gpio099readPrivilegeRstToleranceR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - GPIO100 Read Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio100read_privilege_rst_tolerance(&self) -> Gpio100readPrivilegeRstToleranceR {
        Gpio100readPrivilegeRstToleranceR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - GPIO101 Read Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio101read_privilege_rst_tolerance(&self) -> Gpio101readPrivilegeRstToleranceR {
        Gpio101readPrivilegeRstToleranceR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - GPIO102 Read Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio102read_privilege_rst_tolerance(&self) -> Gpio102readPrivilegeRstToleranceR {
        Gpio102readPrivilegeRstToleranceR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - GPIO103 Read Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio103read_privilege_rst_tolerance(&self) -> Gpio103readPrivilegeRstToleranceR {
        Gpio103readPrivilegeRstToleranceR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - GPIO104 Read Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio104read_privilege_rst_tolerance(&self) -> Gpio104readPrivilegeRstToleranceR {
        Gpio104readPrivilegeRstToleranceR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - GPIO105 Read Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio105read_privilege_rst_tolerance(&self) -> Gpio105readPrivilegeRstToleranceR {
        Gpio105readPrivilegeRstToleranceR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - GPIO106 Read Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio106read_privilege_rst_tolerance(&self) -> Gpio106readPrivilegeRstToleranceR {
        Gpio106readPrivilegeRstToleranceR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - GPIO107 Read Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio107read_privilege_rst_tolerance(&self) -> Gpio107readPrivilegeRstToleranceR {
        Gpio107readPrivilegeRstToleranceR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - GPIO108 Read Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio108read_privilege_rst_tolerance(&self) -> Gpio108readPrivilegeRstToleranceR {
        Gpio108readPrivilegeRstToleranceR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - GPIO109 Read Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio109read_privilege_rst_tolerance(&self) -> Gpio109readPrivilegeRstToleranceR {
        Gpio109readPrivilegeRstToleranceR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - GPIO110 Read Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio110read_privilege_rst_tolerance(&self) -> Gpio110readPrivilegeRstToleranceR {
        Gpio110readPrivilegeRstToleranceR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - GPIO111 Read Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio111read_privilege_rst_tolerance(&self) -> Gpio111readPrivilegeRstToleranceR {
        Gpio111readPrivilegeRstToleranceR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - GPIO112 Read Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio112read_privilege_rst_tolerance(&self) -> Gpio112readPrivilegeRstToleranceR {
        Gpio112readPrivilegeRstToleranceR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - GPIO113 Read Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio113read_privilege_rst_tolerance(&self) -> Gpio113readPrivilegeRstToleranceR {
        Gpio113readPrivilegeRstToleranceR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - GPIO114 Read Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio114read_privilege_rst_tolerance(&self) -> Gpio114readPrivilegeRstToleranceR {
        Gpio114readPrivilegeRstToleranceR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - GPIO115 Read Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio115read_privilege_rst_tolerance(&self) -> Gpio115readPrivilegeRstToleranceR {
        Gpio115readPrivilegeRstToleranceR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - GPIO116 Read Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio116read_privilege_rst_tolerance(&self) -> Gpio116readPrivilegeRstToleranceR {
        Gpio116readPrivilegeRstToleranceR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - GPIO117 Read Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio117read_privilege_rst_tolerance(&self) -> Gpio117readPrivilegeRstToleranceR {
        Gpio117readPrivilegeRstToleranceR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - GPIO118 Read Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio118read_privilege_rst_tolerance(&self) -> Gpio118readPrivilegeRstToleranceR {
        Gpio118readPrivilegeRstToleranceR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - GPIO119 Read Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio119read_privilege_rst_tolerance(&self) -> Gpio119readPrivilegeRstToleranceR {
        Gpio119readPrivilegeRstToleranceR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - GPIO120 Read Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio120read_privilege_rst_tolerance(&self) -> Gpio120readPrivilegeRstToleranceR {
        Gpio120readPrivilegeRstToleranceR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - GPIO121 Read Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio121read_privilege_rst_tolerance(&self) -> Gpio121readPrivilegeRstToleranceR {
        Gpio121readPrivilegeRstToleranceR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - GPIO122 Read Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio122read_privilege_rst_tolerance(&self) -> Gpio122readPrivilegeRstToleranceR {
        Gpio122readPrivilegeRstToleranceR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - GPIO123 Read Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio123read_privilege_rst_tolerance(&self) -> Gpio123readPrivilegeRstToleranceR {
        Gpio123readPrivilegeRstToleranceR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - GPIO124 Read Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio124read_privilege_rst_tolerance(&self) -> Gpio124readPrivilegeRstToleranceR {
        Gpio124readPrivilegeRstToleranceR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - GPIO125 Read Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio125read_privilege_rst_tolerance(&self) -> Gpio125readPrivilegeRstToleranceR {
        Gpio125readPrivilegeRstToleranceR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - GPIO126 Read Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio126read_privilege_rst_tolerance(&self) -> Gpio126readPrivilegeRstToleranceR {
        Gpio126readPrivilegeRstToleranceR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - GPIO127 Read Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio127read_privilege_rst_tolerance(&self) -> Gpio127readPrivilegeRstToleranceR {
        Gpio127readPrivilegeRstToleranceR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - GPIO096 Read Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio096read_privilege_rst_tolerance(
        &mut self,
    ) -> Gpio096readPrivilegeRstToleranceW<Gpioc1cSpec> {
        Gpio096readPrivilegeRstToleranceW::new(self, 0)
    }
    #[doc = "Bit 1 - GPIO097 Read Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio097read_privilege_rst_tolerance(
        &mut self,
    ) -> Gpio097readPrivilegeRstToleranceW<Gpioc1cSpec> {
        Gpio097readPrivilegeRstToleranceW::new(self, 1)
    }
    #[doc = "Bit 2 - GPIO098 Read Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio098read_privilege_rst_tolerance(
        &mut self,
    ) -> Gpio098readPrivilegeRstToleranceW<Gpioc1cSpec> {
        Gpio098readPrivilegeRstToleranceW::new(self, 2)
    }
    #[doc = "Bit 3 - GPIO099 Read Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio099read_privilege_rst_tolerance(
        &mut self,
    ) -> Gpio099readPrivilegeRstToleranceW<Gpioc1cSpec> {
        Gpio099readPrivilegeRstToleranceW::new(self, 3)
    }
    #[doc = "Bit 4 - GPIO100 Read Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio100read_privilege_rst_tolerance(
        &mut self,
    ) -> Gpio100readPrivilegeRstToleranceW<Gpioc1cSpec> {
        Gpio100readPrivilegeRstToleranceW::new(self, 4)
    }
    #[doc = "Bit 5 - GPIO101 Read Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio101read_privilege_rst_tolerance(
        &mut self,
    ) -> Gpio101readPrivilegeRstToleranceW<Gpioc1cSpec> {
        Gpio101readPrivilegeRstToleranceW::new(self, 5)
    }
    #[doc = "Bit 6 - GPIO102 Read Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio102read_privilege_rst_tolerance(
        &mut self,
    ) -> Gpio102readPrivilegeRstToleranceW<Gpioc1cSpec> {
        Gpio102readPrivilegeRstToleranceW::new(self, 6)
    }
    #[doc = "Bit 7 - GPIO103 Read Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio103read_privilege_rst_tolerance(
        &mut self,
    ) -> Gpio103readPrivilegeRstToleranceW<Gpioc1cSpec> {
        Gpio103readPrivilegeRstToleranceW::new(self, 7)
    }
    #[doc = "Bit 8 - GPIO104 Read Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio104read_privilege_rst_tolerance(
        &mut self,
    ) -> Gpio104readPrivilegeRstToleranceW<Gpioc1cSpec> {
        Gpio104readPrivilegeRstToleranceW::new(self, 8)
    }
    #[doc = "Bit 9 - GPIO105 Read Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio105read_privilege_rst_tolerance(
        &mut self,
    ) -> Gpio105readPrivilegeRstToleranceW<Gpioc1cSpec> {
        Gpio105readPrivilegeRstToleranceW::new(self, 9)
    }
    #[doc = "Bit 10 - GPIO106 Read Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio106read_privilege_rst_tolerance(
        &mut self,
    ) -> Gpio106readPrivilegeRstToleranceW<Gpioc1cSpec> {
        Gpio106readPrivilegeRstToleranceW::new(self, 10)
    }
    #[doc = "Bit 11 - GPIO107 Read Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio107read_privilege_rst_tolerance(
        &mut self,
    ) -> Gpio107readPrivilegeRstToleranceW<Gpioc1cSpec> {
        Gpio107readPrivilegeRstToleranceW::new(self, 11)
    }
    #[doc = "Bit 12 - GPIO108 Read Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio108read_privilege_rst_tolerance(
        &mut self,
    ) -> Gpio108readPrivilegeRstToleranceW<Gpioc1cSpec> {
        Gpio108readPrivilegeRstToleranceW::new(self, 12)
    }
    #[doc = "Bit 13 - GPIO109 Read Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio109read_privilege_rst_tolerance(
        &mut self,
    ) -> Gpio109readPrivilegeRstToleranceW<Gpioc1cSpec> {
        Gpio109readPrivilegeRstToleranceW::new(self, 13)
    }
    #[doc = "Bit 14 - GPIO110 Read Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio110read_privilege_rst_tolerance(
        &mut self,
    ) -> Gpio110readPrivilegeRstToleranceW<Gpioc1cSpec> {
        Gpio110readPrivilegeRstToleranceW::new(self, 14)
    }
    #[doc = "Bit 15 - GPIO111 Read Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio111read_privilege_rst_tolerance(
        &mut self,
    ) -> Gpio111readPrivilegeRstToleranceW<Gpioc1cSpec> {
        Gpio111readPrivilegeRstToleranceW::new(self, 15)
    }
    #[doc = "Bit 16 - GPIO112 Read Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio112read_privilege_rst_tolerance(
        &mut self,
    ) -> Gpio112readPrivilegeRstToleranceW<Gpioc1cSpec> {
        Gpio112readPrivilegeRstToleranceW::new(self, 16)
    }
    #[doc = "Bit 17 - GPIO113 Read Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio113read_privilege_rst_tolerance(
        &mut self,
    ) -> Gpio113readPrivilegeRstToleranceW<Gpioc1cSpec> {
        Gpio113readPrivilegeRstToleranceW::new(self, 17)
    }
    #[doc = "Bit 18 - GPIO114 Read Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio114read_privilege_rst_tolerance(
        &mut self,
    ) -> Gpio114readPrivilegeRstToleranceW<Gpioc1cSpec> {
        Gpio114readPrivilegeRstToleranceW::new(self, 18)
    }
    #[doc = "Bit 19 - GPIO115 Read Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio115read_privilege_rst_tolerance(
        &mut self,
    ) -> Gpio115readPrivilegeRstToleranceW<Gpioc1cSpec> {
        Gpio115readPrivilegeRstToleranceW::new(self, 19)
    }
    #[doc = "Bit 20 - GPIO116 Read Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio116read_privilege_rst_tolerance(
        &mut self,
    ) -> Gpio116readPrivilegeRstToleranceW<Gpioc1cSpec> {
        Gpio116readPrivilegeRstToleranceW::new(self, 20)
    }
    #[doc = "Bit 21 - GPIO117 Read Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio117read_privilege_rst_tolerance(
        &mut self,
    ) -> Gpio117readPrivilegeRstToleranceW<Gpioc1cSpec> {
        Gpio117readPrivilegeRstToleranceW::new(self, 21)
    }
    #[doc = "Bit 22 - GPIO118 Read Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio118read_privilege_rst_tolerance(
        &mut self,
    ) -> Gpio118readPrivilegeRstToleranceW<Gpioc1cSpec> {
        Gpio118readPrivilegeRstToleranceW::new(self, 22)
    }
    #[doc = "Bit 23 - GPIO119 Read Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio119read_privilege_rst_tolerance(
        &mut self,
    ) -> Gpio119readPrivilegeRstToleranceW<Gpioc1cSpec> {
        Gpio119readPrivilegeRstToleranceW::new(self, 23)
    }
    #[doc = "Bit 24 - GPIO120 Read Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio120read_privilege_rst_tolerance(
        &mut self,
    ) -> Gpio120readPrivilegeRstToleranceW<Gpioc1cSpec> {
        Gpio120readPrivilegeRstToleranceW::new(self, 24)
    }
    #[doc = "Bit 25 - GPIO121 Read Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio121read_privilege_rst_tolerance(
        &mut self,
    ) -> Gpio121readPrivilegeRstToleranceW<Gpioc1cSpec> {
        Gpio121readPrivilegeRstToleranceW::new(self, 25)
    }
    #[doc = "Bit 26 - GPIO122 Read Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio122read_privilege_rst_tolerance(
        &mut self,
    ) -> Gpio122readPrivilegeRstToleranceW<Gpioc1cSpec> {
        Gpio122readPrivilegeRstToleranceW::new(self, 26)
    }
    #[doc = "Bit 27 - GPIO123 Read Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio123read_privilege_rst_tolerance(
        &mut self,
    ) -> Gpio123readPrivilegeRstToleranceW<Gpioc1cSpec> {
        Gpio123readPrivilegeRstToleranceW::new(self, 27)
    }
    #[doc = "Bit 28 - GPIO124 Read Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio124read_privilege_rst_tolerance(
        &mut self,
    ) -> Gpio124readPrivilegeRstToleranceW<Gpioc1cSpec> {
        Gpio124readPrivilegeRstToleranceW::new(self, 28)
    }
    #[doc = "Bit 29 - GPIO125 Read Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio125read_privilege_rst_tolerance(
        &mut self,
    ) -> Gpio125readPrivilegeRstToleranceW<Gpioc1cSpec> {
        Gpio125readPrivilegeRstToleranceW::new(self, 29)
    }
    #[doc = "Bit 30 - GPIO126 Read Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio126read_privilege_rst_tolerance(
        &mut self,
    ) -> Gpio126readPrivilegeRstToleranceW<Gpioc1cSpec> {
        Gpio126readPrivilegeRstToleranceW::new(self, 30)
    }
    #[doc = "Bit 31 - GPIO127 Read Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio127read_privilege_rst_tolerance(
        &mut self,
    ) -> Gpio127readPrivilegeRstToleranceW<Gpioc1cSpec> {
        Gpio127readPrivilegeRstToleranceW::new(self, 31)
    }
}
#[doc = "Read Privilege Reset Tolerance Register \\#4\n\nYou can [`read`](crate::Reg::read) this register and get [`gpioc1c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpioc1c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gpioc1cSpec;
impl crate::RegisterSpec for Gpioc1cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpioc1c::R`](R) reader structure"]
impl crate::Readable for Gpioc1cSpec {}
#[doc = "`write(|w| ..)` method takes [`gpioc1c::W`](W) writer structure"]
impl crate::Writable for Gpioc1cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GPIOC1C to value 0"]
impl crate::Resettable for Gpioc1cSpec {}
