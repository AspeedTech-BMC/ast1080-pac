#[doc = "Register `SCU440` reader"]
pub type R = crate::R<Scu440Spec>;
#[doc = "Register `SCU440` writer"]
pub type W = crate::W<Scu440Spec>;
#[doc = "Field `SCUMUXIO128` reader - SCU_MUX_IO128"]
pub type Scumuxio128R = crate::FieldReader;
#[doc = "Field `SCUMUXIO128` writer - SCU_MUX_IO128"]
pub type Scumuxio128W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `Reserved7` reader - reserved"]
pub type Reserved7R = crate::BitReader;
#[doc = "Field `SCUMUXIO129` reader - SCU_MUX_IO129"]
pub type Scumuxio129R = crate::FieldReader;
#[doc = "Field `SCUMUXIO129` writer - SCU_MUX_IO129"]
pub type Scumuxio129W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `Reserved6` reader - reserved"]
pub type Reserved6R = crate::BitReader;
#[doc = "Field `SCUMUXIO130` reader - SCU_MUX_IO130"]
pub type Scumuxio130R = crate::FieldReader;
#[doc = "Field `SCUMUXIO130` writer - SCU_MUX_IO130"]
pub type Scumuxio130W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `Reserved5` reader - reserved"]
pub type Reserved5R = crate::BitReader;
#[doc = "Field `SCUMUXIO131` reader - SCU_MUX_IO131"]
pub type Scumuxio131R = crate::FieldReader;
#[doc = "Field `SCUMUXIO131` writer - SCU_MUX_IO131"]
pub type Scumuxio131W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `Reserved4` reader - reserved"]
pub type Reserved4R = crate::BitReader;
#[doc = "Field `SCUMUXIO132` reader - SCU_MUX_IO132"]
pub type Scumuxio132R = crate::FieldReader;
#[doc = "Field `SCUMUXIO132` writer - SCU_MUX_IO132"]
pub type Scumuxio132W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `Reserved3` reader - reserved"]
pub type Reserved3R = crate::BitReader;
#[doc = "Field `SCUMUXIO133` reader - SCU_MUX_IO133"]
pub type Scumuxio133R = crate::FieldReader;
#[doc = "Field `SCUMUXIO133` writer - SCU_MUX_IO133"]
pub type Scumuxio133W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `Reserved2` reader - reserved"]
pub type Reserved2R = crate::BitReader;
#[doc = "Field `SCUMUXIO134` reader - SCU_MUX_IO134"]
pub type Scumuxio134R = crate::FieldReader;
#[doc = "Field `SCUMUXIO134` writer - SCU_MUX_IO134"]
pub type Scumuxio134W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `Reserved1` reader - reserved"]
pub type Reserved1R = crate::BitReader;
#[doc = "Field `SCUMUXIO135` reader - SCU_MUX_IO135"]
pub type Scumuxio135R = crate::FieldReader;
#[doc = "Field `SCUMUXIO135` writer - SCU_MUX_IO135"]
pub type Scumuxio135W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2 - SCU_MUX_IO128"]
    #[inline(always)]
    pub fn scumuxio128(&self) -> Scumuxio128R {
        Scumuxio128R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3 - reserved"]
    #[inline(always)]
    pub fn reserved7(&self) -> Reserved7R {
        Reserved7R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:6 - SCU_MUX_IO129"]
    #[inline(always)]
    pub fn scumuxio129(&self) -> Scumuxio129R {
        Scumuxio129R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 7 - reserved"]
    #[inline(always)]
    pub fn reserved6(&self) -> Reserved6R {
        Reserved6R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:10 - SCU_MUX_IO130"]
    #[inline(always)]
    pub fn scumuxio130(&self) -> Scumuxio130R {
        Scumuxio130R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bit 11 - reserved"]
    #[inline(always)]
    pub fn reserved5(&self) -> Reserved5R {
        Reserved5R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:14 - SCU_MUX_IO131"]
    #[inline(always)]
    pub fn scumuxio131(&self) -> Scumuxio131R {
        Scumuxio131R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bit 15 - reserved"]
    #[inline(always)]
    pub fn reserved4(&self) -> Reserved4R {
        Reserved4R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:18 - SCU_MUX_IO132"]
    #[inline(always)]
    pub fn scumuxio132(&self) -> Scumuxio132R {
        Scumuxio132R::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bit 19 - reserved"]
    #[inline(always)]
    pub fn reserved3(&self) -> Reserved3R {
        Reserved3R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 20:22 - SCU_MUX_IO133"]
    #[inline(always)]
    pub fn scumuxio133(&self) -> Scumuxio133R {
        Scumuxio133R::new(((self.bits >> 20) & 7) as u8)
    }
    #[doc = "Bit 23 - reserved"]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:26 - SCU_MUX_IO134"]
    #[inline(always)]
    pub fn scumuxio134(&self) -> Scumuxio134R {
        Scumuxio134R::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bit 27 - reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bits 28:30 - SCU_MUX_IO135"]
    #[inline(always)]
    pub fn scumuxio135(&self) -> Scumuxio135R {
        Scumuxio135R::new(((self.bits >> 28) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - SCU_MUX_IO128"]
    #[inline(always)]
    pub fn scumuxio128(&mut self) -> Scumuxio128W<Scu440Spec> {
        Scumuxio128W::new(self, 0)
    }
    #[doc = "Bits 4:6 - SCU_MUX_IO129"]
    #[inline(always)]
    pub fn scumuxio129(&mut self) -> Scumuxio129W<Scu440Spec> {
        Scumuxio129W::new(self, 4)
    }
    #[doc = "Bits 8:10 - SCU_MUX_IO130"]
    #[inline(always)]
    pub fn scumuxio130(&mut self) -> Scumuxio130W<Scu440Spec> {
        Scumuxio130W::new(self, 8)
    }
    #[doc = "Bits 12:14 - SCU_MUX_IO131"]
    #[inline(always)]
    pub fn scumuxio131(&mut self) -> Scumuxio131W<Scu440Spec> {
        Scumuxio131W::new(self, 12)
    }
    #[doc = "Bits 16:18 - SCU_MUX_IO132"]
    #[inline(always)]
    pub fn scumuxio132(&mut self) -> Scumuxio132W<Scu440Spec> {
        Scumuxio132W::new(self, 16)
    }
    #[doc = "Bits 20:22 - SCU_MUX_IO133"]
    #[inline(always)]
    pub fn scumuxio133(&mut self) -> Scumuxio133W<Scu440Spec> {
        Scumuxio133W::new(self, 20)
    }
    #[doc = "Bits 24:26 - SCU_MUX_IO134"]
    #[inline(always)]
    pub fn scumuxio134(&mut self) -> Scumuxio134W<Scu440Spec> {
        Scumuxio134W::new(self, 24)
    }
    #[doc = "Bits 28:30 - SCU_MUX_IO135"]
    #[inline(always)]
    pub fn scumuxio135(&mut self) -> Scumuxio135W<Scu440Spec> {
        Scumuxio135W::new(self, 28)
    }
}
#[doc = "Multi-Function Pin Control \\#17\n\nYou can [`read`](crate::Reg::read) this register and get [`scu440::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu440::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scu440Spec;
impl crate::RegisterSpec for Scu440Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scu440::R`](R) reader structure"]
impl crate::Readable for Scu440Spec {}
#[doc = "`write(|w| ..)` method takes [`scu440::W`](W) writer structure"]
impl crate::Writable for Scu440Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCU440 to value 0x2211_1111"]
impl crate::Resettable for Scu440Spec {
    const RESET_VALUE: u32 = 0x2211_1111;
}
