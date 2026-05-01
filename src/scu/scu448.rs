#[doc = "Register `SCU448` reader"]
pub type R = crate::R<Scu448Spec>;
#[doc = "Register `SCU448` writer"]
pub type W = crate::W<Scu448Spec>;
#[doc = "Field `SCUMUXIO144` reader - SCU_MUX_IO144"]
pub type Scumuxio144R = crate::FieldReader;
#[doc = "Field `SCUMUXIO144` writer - SCU_MUX_IO144"]
pub type Scumuxio144W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `Reserved7` reader - reserved"]
pub type Reserved7R = crate::BitReader;
#[doc = "Field `SCUMUXIO145` reader - SCU_MUX_IO145"]
pub type Scumuxio145R = crate::FieldReader;
#[doc = "Field `SCUMUXIO145` writer - SCU_MUX_IO145"]
pub type Scumuxio145W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `Reserved6` reader - reserved"]
pub type Reserved6R = crate::BitReader;
#[doc = "Field `SCUMUXIO146` reader - SCU_MUX_IO146"]
pub type Scumuxio146R = crate::FieldReader;
#[doc = "Field `SCUMUXIO146` writer - SCU_MUX_IO146"]
pub type Scumuxio146W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `Reserved5` reader - reserved"]
pub type Reserved5R = crate::BitReader;
#[doc = "Field `SCUMUXIO147` reader - SCU_MUX_IO147"]
pub type Scumuxio147R = crate::FieldReader;
#[doc = "Field `SCUMUXIO147` writer - SCU_MUX_IO147"]
pub type Scumuxio147W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `Reserved4` reader - reserved"]
pub type Reserved4R = crate::BitReader;
#[doc = "Field `SCUMUXIO148` reader - SCU_MUX_IO148"]
pub type Scumuxio148R = crate::FieldReader;
#[doc = "Field `SCUMUXIO148` writer - SCU_MUX_IO148"]
pub type Scumuxio148W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `Reserved3` reader - reserved"]
pub type Reserved3R = crate::BitReader;
#[doc = "Field `SCUMUXIO149` reader - SCU_MUX_IO149"]
pub type Scumuxio149R = crate::FieldReader;
#[doc = "Field `SCUMUXIO149` writer - SCU_MUX_IO149"]
pub type Scumuxio149W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `Reserved2` reader - reserved"]
pub type Reserved2R = crate::BitReader;
#[doc = "Field `SCUMUXIO150` reader - SCU_MUX_IO150"]
pub type Scumuxio150R = crate::FieldReader;
#[doc = "Field `SCUMUXIO150` writer - SCU_MUX_IO150"]
pub type Scumuxio150W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `Reserved1` reader - reserved"]
pub type Reserved1R = crate::BitReader;
#[doc = "Field `SCUMUXIO151` reader - SCU_MUX_IO151"]
pub type Scumuxio151R = crate::FieldReader;
#[doc = "Field `SCUMUXIO151` writer - SCU_MUX_IO151"]
pub type Scumuxio151W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2 - SCU_MUX_IO144"]
    #[inline(always)]
    pub fn scumuxio144(&self) -> Scumuxio144R {
        Scumuxio144R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3 - reserved"]
    #[inline(always)]
    pub fn reserved7(&self) -> Reserved7R {
        Reserved7R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:6 - SCU_MUX_IO145"]
    #[inline(always)]
    pub fn scumuxio145(&self) -> Scumuxio145R {
        Scumuxio145R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 7 - reserved"]
    #[inline(always)]
    pub fn reserved6(&self) -> Reserved6R {
        Reserved6R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:10 - SCU_MUX_IO146"]
    #[inline(always)]
    pub fn scumuxio146(&self) -> Scumuxio146R {
        Scumuxio146R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bit 11 - reserved"]
    #[inline(always)]
    pub fn reserved5(&self) -> Reserved5R {
        Reserved5R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:14 - SCU_MUX_IO147"]
    #[inline(always)]
    pub fn scumuxio147(&self) -> Scumuxio147R {
        Scumuxio147R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bit 15 - reserved"]
    #[inline(always)]
    pub fn reserved4(&self) -> Reserved4R {
        Reserved4R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:18 - SCU_MUX_IO148"]
    #[inline(always)]
    pub fn scumuxio148(&self) -> Scumuxio148R {
        Scumuxio148R::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bit 19 - reserved"]
    #[inline(always)]
    pub fn reserved3(&self) -> Reserved3R {
        Reserved3R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 20:22 - SCU_MUX_IO149"]
    #[inline(always)]
    pub fn scumuxio149(&self) -> Scumuxio149R {
        Scumuxio149R::new(((self.bits >> 20) & 7) as u8)
    }
    #[doc = "Bit 23 - reserved"]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:26 - SCU_MUX_IO150"]
    #[inline(always)]
    pub fn scumuxio150(&self) -> Scumuxio150R {
        Scumuxio150R::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bit 27 - reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bits 28:30 - SCU_MUX_IO151"]
    #[inline(always)]
    pub fn scumuxio151(&self) -> Scumuxio151R {
        Scumuxio151R::new(((self.bits >> 28) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - SCU_MUX_IO144"]
    #[inline(always)]
    pub fn scumuxio144(&mut self) -> Scumuxio144W<Scu448Spec> {
        Scumuxio144W::new(self, 0)
    }
    #[doc = "Bits 4:6 - SCU_MUX_IO145"]
    #[inline(always)]
    pub fn scumuxio145(&mut self) -> Scumuxio145W<Scu448Spec> {
        Scumuxio145W::new(self, 4)
    }
    #[doc = "Bits 8:10 - SCU_MUX_IO146"]
    #[inline(always)]
    pub fn scumuxio146(&mut self) -> Scumuxio146W<Scu448Spec> {
        Scumuxio146W::new(self, 8)
    }
    #[doc = "Bits 12:14 - SCU_MUX_IO147"]
    #[inline(always)]
    pub fn scumuxio147(&mut self) -> Scumuxio147W<Scu448Spec> {
        Scumuxio147W::new(self, 12)
    }
    #[doc = "Bits 16:18 - SCU_MUX_IO148"]
    #[inline(always)]
    pub fn scumuxio148(&mut self) -> Scumuxio148W<Scu448Spec> {
        Scumuxio148W::new(self, 16)
    }
    #[doc = "Bits 20:22 - SCU_MUX_IO149"]
    #[inline(always)]
    pub fn scumuxio149(&mut self) -> Scumuxio149W<Scu448Spec> {
        Scumuxio149W::new(self, 20)
    }
    #[doc = "Bits 24:26 - SCU_MUX_IO150"]
    #[inline(always)]
    pub fn scumuxio150(&mut self) -> Scumuxio150W<Scu448Spec> {
        Scumuxio150W::new(self, 24)
    }
    #[doc = "Bits 28:30 - SCU_MUX_IO151"]
    #[inline(always)]
    pub fn scumuxio151(&mut self) -> Scumuxio151W<Scu448Spec> {
        Scumuxio151W::new(self, 28)
    }
}
#[doc = "Multi-Function Pin Control \\#19\n\nYou can [`read`](crate::Reg::read) this register and get [`scu448::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu448::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scu448Spec;
impl crate::RegisterSpec for Scu448Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scu448::R`](R) reader structure"]
impl crate::Readable for Scu448Spec {}
#[doc = "`write(|w| ..)` method takes [`scu448::W`](W) writer structure"]
impl crate::Writable for Scu448Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCU448 to value 0"]
impl crate::Resettable for Scu448Spec {}
