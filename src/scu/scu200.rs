#[doc = "Register `SCU200` reader"]
pub type R = crate::R<Scu200Spec>;
#[doc = "Register `SCU200` writer"]
pub type W = crate::W<Scu200Spec>;
#[doc = "Field `SCURESETLSC` reader - SCU_RESET_LSC"]
pub type ScuresetlscR = crate::BitReader;
#[doc = "Field `SCURESETLSC` writer - SCU_RESET_LSC"]
pub type ScuresetlscW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved5` reader - reserved"]
pub type Reserved5R = crate::BitReader;
#[doc = "Field `Reserved3` reader - reserved"]
pub type Reserved3R = crate::BitReader;
#[doc = "Field `Reserved3` writer - reserved"]
pub type Reserved3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved4` reader - reserved"]
pub type Reserved4R = crate::FieldReader;
#[doc = "Field `Reserved4` writer - reserved"]
pub type Reserved4W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `SCURESETPECI` reader - SCU_RESET_PECI"]
pub type ScuresetpeciR = crate::BitReader;
#[doc = "Field `SCURESETPECI` writer - SCU_RESET_PECI"]
pub type ScuresetpeciW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCURESETPWM` reader - SCU_RESET_PWM"]
pub type ScuresetpwmR = crate::BitReader;
#[doc = "Field `SCURESETPWM` writer - SCU_RESET_PWM"]
pub type ScuresetpwmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved2` reader - reserved"]
pub type Reserved2R = crate::FieldReader;
#[doc = "Field `Reserved2` writer - reserved"]
pub type Reserved2W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SCURESETMEMC` reader - SCU_RESET_MEMC"]
pub type ScuresetmemcR = crate::BitReader;
#[doc = "Field `SCURESETMEMC` writer - SCU_RESET_MEMC"]
pub type ScuresetmemcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCURESETADC` reader - SCU_RESET_ADC"]
pub type ScuresetadcR = crate::BitReader;
#[doc = "Field `SCURESETADC` writer - SCU_RESET_ADC"]
pub type ScuresetadcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCURESETMCU` reader - SCU_RESET_MCU"]
pub type ScuresetmcuR = crate::BitReader;
#[doc = "Field `SCURESETMCU` writer - SCU_RESET_MCU"]
pub type ScuresetmcuW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCURESETESPI` reader - SCU_RESET_ESPI"]
pub type ScuresetespiR = crate::BitReader;
#[doc = "Field `SCURESETESPI` writer - SCU_RESET_ESPI"]
pub type ScuresetespiW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved1` reader - reserved"]
pub type Reserved1R = crate::BitReader;
#[doc = "Field `Reserved1` writer - reserved"]
pub type Reserved1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCURESETCPTRA` reader - SCU_RESET_CPTRA"]
pub type ScuresetcptraR = crate::BitReader;
#[doc = "Field `SCURESETCPTRA` writer - SCU_RESET_CPTRA"]
pub type ScuresetcptraW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCURESETJTAG0` reader - SCU_RESET_JTAG0"]
pub type Scuresetjtag0R = crate::BitReader;
#[doc = "Field `SCURESETJTAG0` writer - SCU_RESET_JTAG0"]
pub type Scuresetjtag0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCURESETJTAG1` reader - SCU_RESET_JTAG1"]
pub type Scuresetjtag1R = crate::BitReader;
#[doc = "Field `SCURESETJTAG1` writer - SCU_RESET_JTAG1"]
pub type Scuresetjtag1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCURESETSPI0` reader - SCU_RESET_SPI0"]
pub type Scuresetspi0R = crate::BitReader;
#[doc = "Field `SCURESETSPI0` writer - SCU_RESET_SPI0"]
pub type Scuresetspi0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCURESETSPI1` reader - SCU_RESET_SPI1"]
pub type Scuresetspi1R = crate::BitReader;
#[doc = "Field `SCURESETSPI1` writer - SCU_RESET_SPI1"]
pub type Scuresetspi1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCURESETI3C0` reader - SCU_RESET_I3C0"]
pub type Scureseti3c0R = crate::BitReader;
#[doc = "Field `SCURESETI3C0` writer - SCU_RESET_I3C0"]
pub type Scureseti3c0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCURESETI3C1` reader - SCU_RESET_I3C1"]
pub type Scureseti3c1R = crate::BitReader;
#[doc = "Field `SCURESETI3C1` writer - SCU_RESET_I3C1"]
pub type Scureseti3c1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCURESETI3C2` reader - SCU_RESET_I3C2"]
pub type Scureseti3c2R = crate::BitReader;
#[doc = "Field `SCURESETI3C2` writer - SCU_RESET_I3C2"]
pub type Scureseti3c2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCURESETI3C3` reader - SCU_RESET_I3C3"]
pub type Scureseti3c3R = crate::BitReader;
#[doc = "Field `SCURESETI3C3` writer - SCU_RESET_I3C3"]
pub type Scureseti3c3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCURESETI3C4` reader - SCU_RESET_I3C4"]
pub type Scureseti3c4R = crate::BitReader;
#[doc = "Field `SCURESETI3C4` writer - SCU_RESET_I3C4"]
pub type Scureseti3c4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCURESETI3C5` reader - SCU_RESET_I3C5"]
pub type Scureseti3c5R = crate::BitReader;
#[doc = "Field `SCURESETI3C5` writer - SCU_RESET_I3C5"]
pub type Scureseti3c5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCURESETI3C6` reader - SCU_RESET_I3C6"]
pub type Scureseti3c6R = crate::BitReader;
#[doc = "Field `SCURESETI3C6` writer - SCU_RESET_I3C6"]
pub type Scureseti3c6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCURESETI3C7` reader - SCU_RESET_I3C7"]
pub type Scureseti3c7R = crate::BitReader;
#[doc = "Field `SCURESETI3C7` writer - SCU_RESET_I3C7"]
pub type Scureseti3c7W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - SCU_RESET_LSC"]
    #[inline(always)]
    pub fn scuresetlsc(&self) -> ScuresetlscR {
        ScuresetlscR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - reserved"]
    #[inline(always)]
    pub fn reserved5(&self) -> Reserved5R {
        Reserved5R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - reserved"]
    #[inline(always)]
    pub fn reserved3(&self) -> Reserved3R {
        Reserved3R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 2:4 - reserved"]
    #[inline(always)]
    pub fn reserved4(&self) -> Reserved4R {
        Reserved4R::new(((self.bits >> 2) & 7) as u8)
    }
    #[doc = "Bit 3 - SCU_RESET_PECI"]
    #[inline(always)]
    pub fn scuresetpeci(&self) -> ScuresetpeciR {
        ScuresetpeciR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - SCU_RESET_PWM"]
    #[inline(always)]
    pub fn scuresetpwm(&self) -> ScuresetpwmR {
        ScuresetpwmR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:6 - reserved"]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new(((self.bits >> 5) & 3) as u8)
    }
    #[doc = "Bit 7 - SCU_RESET_MEMC"]
    #[inline(always)]
    pub fn scuresetmemc(&self) -> ScuresetmemcR {
        ScuresetmemcR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - SCU_RESET_ADC"]
    #[inline(always)]
    pub fn scuresetadc(&self) -> ScuresetadcR {
        ScuresetadcR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - SCU_RESET_MCU"]
    #[inline(always)]
    pub fn scuresetmcu(&self) -> ScuresetmcuR {
        ScuresetmcuR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - SCU_RESET_ESPI"]
    #[inline(always)]
    pub fn scuresetespi(&self) -> ScuresetespiR {
        ScuresetespiR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 10 - reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - SCU_RESET_CPTRA"]
    #[inline(always)]
    pub fn scuresetcptra(&self) -> ScuresetcptraR {
        ScuresetcptraR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - SCU_RESET_JTAG0"]
    #[inline(always)]
    pub fn scuresetjtag0(&self) -> Scuresetjtag0R {
        Scuresetjtag0R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - SCU_RESET_JTAG1"]
    #[inline(always)]
    pub fn scuresetjtag1(&self) -> Scuresetjtag1R {
        Scuresetjtag1R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - SCU_RESET_SPI0"]
    #[inline(always)]
    pub fn scuresetspi0(&self) -> Scuresetspi0R {
        Scuresetspi0R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - SCU_RESET_SPI1"]
    #[inline(always)]
    pub fn scuresetspi1(&self) -> Scuresetspi1R {
        Scuresetspi1R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - SCU_RESET_I3C0"]
    #[inline(always)]
    pub fn scureseti3c0(&self) -> Scureseti3c0R {
        Scureseti3c0R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - SCU_RESET_I3C1"]
    #[inline(always)]
    pub fn scureseti3c1(&self) -> Scureseti3c1R {
        Scureseti3c1R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - SCU_RESET_I3C2"]
    #[inline(always)]
    pub fn scureseti3c2(&self) -> Scureseti3c2R {
        Scureseti3c2R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - SCU_RESET_I3C3"]
    #[inline(always)]
    pub fn scureseti3c3(&self) -> Scureseti3c3R {
        Scureseti3c3R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - SCU_RESET_I3C4"]
    #[inline(always)]
    pub fn scureseti3c4(&self) -> Scureseti3c4R {
        Scureseti3c4R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - SCU_RESET_I3C5"]
    #[inline(always)]
    pub fn scureseti3c5(&self) -> Scureseti3c5R {
        Scureseti3c5R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - SCU_RESET_I3C6"]
    #[inline(always)]
    pub fn scureseti3c6(&self) -> Scureseti3c6R {
        Scureseti3c6R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - SCU_RESET_I3C7"]
    #[inline(always)]
    pub fn scureseti3c7(&self) -> Scureseti3c7R {
        Scureseti3c7R::new(((self.bits >> 23) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SCU_RESET_LSC"]
    #[inline(always)]
    pub fn scuresetlsc(&mut self) -> ScuresetlscW<Scu200Spec> {
        ScuresetlscW::new(self, 0)
    }
    #[doc = "Bit 2 - reserved"]
    #[inline(always)]
    pub fn reserved3(&mut self) -> Reserved3W<Scu200Spec> {
        Reserved3W::new(self, 2)
    }
    #[doc = "Bits 2:4 - reserved"]
    #[inline(always)]
    pub fn reserved4(&mut self) -> Reserved4W<Scu200Spec> {
        Reserved4W::new(self, 2)
    }
    #[doc = "Bit 3 - SCU_RESET_PECI"]
    #[inline(always)]
    pub fn scuresetpeci(&mut self) -> ScuresetpeciW<Scu200Spec> {
        ScuresetpeciW::new(self, 3)
    }
    #[doc = "Bit 4 - SCU_RESET_PWM"]
    #[inline(always)]
    pub fn scuresetpwm(&mut self) -> ScuresetpwmW<Scu200Spec> {
        ScuresetpwmW::new(self, 4)
    }
    #[doc = "Bits 5:6 - reserved"]
    #[inline(always)]
    pub fn reserved2(&mut self) -> Reserved2W<Scu200Spec> {
        Reserved2W::new(self, 5)
    }
    #[doc = "Bit 7 - SCU_RESET_MEMC"]
    #[inline(always)]
    pub fn scuresetmemc(&mut self) -> ScuresetmemcW<Scu200Spec> {
        ScuresetmemcW::new(self, 7)
    }
    #[doc = "Bit 8 - SCU_RESET_ADC"]
    #[inline(always)]
    pub fn scuresetadc(&mut self) -> ScuresetadcW<Scu200Spec> {
        ScuresetadcW::new(self, 8)
    }
    #[doc = "Bit 9 - SCU_RESET_MCU"]
    #[inline(always)]
    pub fn scuresetmcu(&mut self) -> ScuresetmcuW<Scu200Spec> {
        ScuresetmcuW::new(self, 9)
    }
    #[doc = "Bit 10 - SCU_RESET_ESPI"]
    #[inline(always)]
    pub fn scuresetespi(&mut self) -> ScuresetespiW<Scu200Spec> {
        ScuresetespiW::new(self, 10)
    }
    #[doc = "Bit 10 - reserved"]
    #[inline(always)]
    pub fn reserved1(&mut self) -> Reserved1W<Scu200Spec> {
        Reserved1W::new(self, 10)
    }
    #[doc = "Bit 11 - SCU_RESET_CPTRA"]
    #[inline(always)]
    pub fn scuresetcptra(&mut self) -> ScuresetcptraW<Scu200Spec> {
        ScuresetcptraW::new(self, 11)
    }
    #[doc = "Bit 12 - SCU_RESET_JTAG0"]
    #[inline(always)]
    pub fn scuresetjtag0(&mut self) -> Scuresetjtag0W<Scu200Spec> {
        Scuresetjtag0W::new(self, 12)
    }
    #[doc = "Bit 13 - SCU_RESET_JTAG1"]
    #[inline(always)]
    pub fn scuresetjtag1(&mut self) -> Scuresetjtag1W<Scu200Spec> {
        Scuresetjtag1W::new(self, 13)
    }
    #[doc = "Bit 14 - SCU_RESET_SPI0"]
    #[inline(always)]
    pub fn scuresetspi0(&mut self) -> Scuresetspi0W<Scu200Spec> {
        Scuresetspi0W::new(self, 14)
    }
    #[doc = "Bit 15 - SCU_RESET_SPI1"]
    #[inline(always)]
    pub fn scuresetspi1(&mut self) -> Scuresetspi1W<Scu200Spec> {
        Scuresetspi1W::new(self, 15)
    }
    #[doc = "Bit 16 - SCU_RESET_I3C0"]
    #[inline(always)]
    pub fn scureseti3c0(&mut self) -> Scureseti3c0W<Scu200Spec> {
        Scureseti3c0W::new(self, 16)
    }
    #[doc = "Bit 17 - SCU_RESET_I3C1"]
    #[inline(always)]
    pub fn scureseti3c1(&mut self) -> Scureseti3c1W<Scu200Spec> {
        Scureseti3c1W::new(self, 17)
    }
    #[doc = "Bit 18 - SCU_RESET_I3C2"]
    #[inline(always)]
    pub fn scureseti3c2(&mut self) -> Scureseti3c2W<Scu200Spec> {
        Scureseti3c2W::new(self, 18)
    }
    #[doc = "Bit 19 - SCU_RESET_I3C3"]
    #[inline(always)]
    pub fn scureseti3c3(&mut self) -> Scureseti3c3W<Scu200Spec> {
        Scureseti3c3W::new(self, 19)
    }
    #[doc = "Bit 20 - SCU_RESET_I3C4"]
    #[inline(always)]
    pub fn scureseti3c4(&mut self) -> Scureseti3c4W<Scu200Spec> {
        Scureseti3c4W::new(self, 20)
    }
    #[doc = "Bit 21 - SCU_RESET_I3C5"]
    #[inline(always)]
    pub fn scureseti3c5(&mut self) -> Scureseti3c5W<Scu200Spec> {
        Scureseti3c5W::new(self, 21)
    }
    #[doc = "Bit 22 - SCU_RESET_I3C6"]
    #[inline(always)]
    pub fn scureseti3c6(&mut self) -> Scureseti3c6W<Scu200Spec> {
        Scureseti3c6W::new(self, 22)
    }
    #[doc = "Bit 23 - SCU_RESET_I3C7"]
    #[inline(always)]
    pub fn scureseti3c7(&mut self) -> Scureseti3c7W<Scu200Spec> {
        Scureseti3c7W::new(self, 23)
    }
}
#[doc = "System Reset Control 1 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`scu200::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu200::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scu200Spec;
impl crate::RegisterSpec for Scu200Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scu200::R`](R) reader structure"]
impl crate::Readable for Scu200Spec {}
#[doc = "`write(|w| ..)` method takes [`scu200::W`](W) writer structure"]
impl crate::Writable for Scu200Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCU200 to value 0x00ff_317c"]
impl crate::Resettable for Scu200Spec {
    const RESET_VALUE: u32 = 0x00ff_317c;
}
