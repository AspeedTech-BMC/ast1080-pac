#[doc = "Register `SCU458` reader"]
pub type R = crate::R<Scu458Spec>;
#[doc = "Register `SCU458` writer"]
pub type W = crate::W<Scu458Spec>;
#[doc = "Field `SCUMUXIO176` reader - SCU_MUX_IO176"]
pub type Scumuxio176R = crate::FieldReader;
#[doc = "Field `SCUMUXIO176` writer - SCU_MUX_IO176"]
pub type Scumuxio176W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `Reserved7` reader - reserved"]
pub type Reserved7R = crate::BitReader;
#[doc = "Field `SCUMUXIO177` reader - SCU_MUX_IO177"]
pub type Scumuxio177R = crate::FieldReader;
#[doc = "Field `SCUMUXIO177` writer - SCU_MUX_IO177"]
pub type Scumuxio177W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `Reserved6` reader - reserved"]
pub type Reserved6R = crate::BitReader;
#[doc = "Field `SCUMUXIO178` reader - SCU_MUX_IO178"]
pub type Scumuxio178R = crate::FieldReader;
#[doc = "Field `SCUMUXIO178` writer - SCU_MUX_IO178"]
pub type Scumuxio178W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `Reserved5` reader - reserved"]
pub type Reserved5R = crate::BitReader;
#[doc = "Field `SCUMUXIO179` reader - SCU_MUX_IO179"]
pub type Scumuxio179R = crate::FieldReader;
#[doc = "Field `SCUMUXIO179` writer - SCU_MUX_IO179"]
pub type Scumuxio179W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `Reserved4` reader - reserved"]
pub type Reserved4R = crate::BitReader;
#[doc = "Field `SCUMUXIO180` reader - SCU_MUX_IO180"]
pub type Scumuxio180R = crate::FieldReader;
#[doc = "Field `SCUMUXIO180` writer - SCU_MUX_IO180"]
pub type Scumuxio180W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `Reserved3` reader - reserved"]
pub type Reserved3R = crate::BitReader;
#[doc = "Field `SCUMUXIO181` reader - SCU_MUX_IO181"]
pub type Scumuxio181R = crate::FieldReader;
#[doc = "Field `SCUMUXIO181` writer - SCU_MUX_IO181"]
pub type Scumuxio181W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `Reserved2` reader - reserved"]
pub type Reserved2R = crate::BitReader;
#[doc = "Field `SCUMUXIO182` reader - SCU_MUX_IO182"]
pub type Scumuxio182R = crate::FieldReader;
#[doc = "Field `SCUMUXIO182` writer - SCU_MUX_IO182"]
pub type Scumuxio182W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `Reserved1` reader - reserved"]
pub type Reserved1R = crate::BitReader;
#[doc = "Field `SCUMUXIO183` reader - SCU_MUX_IO183"]
pub type Scumuxio183R = crate::FieldReader;
#[doc = "Field `SCUMUXIO183` writer - SCU_MUX_IO183"]
pub type Scumuxio183W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2 - SCU_MUX_IO176"]
    #[inline(always)]
    pub fn scumuxio176(&self) -> Scumuxio176R {
        Scumuxio176R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3 - reserved"]
    #[inline(always)]
    pub fn reserved7(&self) -> Reserved7R {
        Reserved7R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:6 - SCU_MUX_IO177"]
    #[inline(always)]
    pub fn scumuxio177(&self) -> Scumuxio177R {
        Scumuxio177R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 7 - reserved"]
    #[inline(always)]
    pub fn reserved6(&self) -> Reserved6R {
        Reserved6R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:10 - SCU_MUX_IO178"]
    #[inline(always)]
    pub fn scumuxio178(&self) -> Scumuxio178R {
        Scumuxio178R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bit 11 - reserved"]
    #[inline(always)]
    pub fn reserved5(&self) -> Reserved5R {
        Reserved5R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:14 - SCU_MUX_IO179"]
    #[inline(always)]
    pub fn scumuxio179(&self) -> Scumuxio179R {
        Scumuxio179R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bit 15 - reserved"]
    #[inline(always)]
    pub fn reserved4(&self) -> Reserved4R {
        Reserved4R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:18 - SCU_MUX_IO180"]
    #[inline(always)]
    pub fn scumuxio180(&self) -> Scumuxio180R {
        Scumuxio180R::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bit 19 - reserved"]
    #[inline(always)]
    pub fn reserved3(&self) -> Reserved3R {
        Reserved3R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 20:22 - SCU_MUX_IO181"]
    #[inline(always)]
    pub fn scumuxio181(&self) -> Scumuxio181R {
        Scumuxio181R::new(((self.bits >> 20) & 7) as u8)
    }
    #[doc = "Bit 23 - reserved"]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:26 - SCU_MUX_IO182"]
    #[inline(always)]
    pub fn scumuxio182(&self) -> Scumuxio182R {
        Scumuxio182R::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bit 27 - reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bits 28:30 - SCU_MUX_IO183"]
    #[inline(always)]
    pub fn scumuxio183(&self) -> Scumuxio183R {
        Scumuxio183R::new(((self.bits >> 28) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - SCU_MUX_IO176"]
    #[inline(always)]
    pub fn scumuxio176(&mut self) -> Scumuxio176W<Scu458Spec> {
        Scumuxio176W::new(self, 0)
    }
    #[doc = "Bits 4:6 - SCU_MUX_IO177"]
    #[inline(always)]
    pub fn scumuxio177(&mut self) -> Scumuxio177W<Scu458Spec> {
        Scumuxio177W::new(self, 4)
    }
    #[doc = "Bits 8:10 - SCU_MUX_IO178"]
    #[inline(always)]
    pub fn scumuxio178(&mut self) -> Scumuxio178W<Scu458Spec> {
        Scumuxio178W::new(self, 8)
    }
    #[doc = "Bits 12:14 - SCU_MUX_IO179"]
    #[inline(always)]
    pub fn scumuxio179(&mut self) -> Scumuxio179W<Scu458Spec> {
        Scumuxio179W::new(self, 12)
    }
    #[doc = "Bits 16:18 - SCU_MUX_IO180"]
    #[inline(always)]
    pub fn scumuxio180(&mut self) -> Scumuxio180W<Scu458Spec> {
        Scumuxio180W::new(self, 16)
    }
    #[doc = "Bits 20:22 - SCU_MUX_IO181"]
    #[inline(always)]
    pub fn scumuxio181(&mut self) -> Scumuxio181W<Scu458Spec> {
        Scumuxio181W::new(self, 20)
    }
    #[doc = "Bits 24:26 - SCU_MUX_IO182"]
    #[inline(always)]
    pub fn scumuxio182(&mut self) -> Scumuxio182W<Scu458Spec> {
        Scumuxio182W::new(self, 24)
    }
    #[doc = "Bits 28:30 - SCU_MUX_IO183"]
    #[inline(always)]
    pub fn scumuxio183(&mut self) -> Scumuxio183W<Scu458Spec> {
        Scumuxio183W::new(self, 28)
    }
}
#[doc = "Multi-Function Pin Control \\#23\n\nYou can [`read`](crate::Reg::read) this register and get [`scu458::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu458::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scu458Spec;
impl crate::RegisterSpec for Scu458Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scu458::R`](R) reader structure"]
impl crate::Readable for Scu458Spec {}
#[doc = "`write(|w| ..)` method takes [`scu458::W`](W) writer structure"]
impl crate::Writable for Scu458Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCU458 to value 0x1111_1131"]
impl crate::Resettable for Scu458Spec {
    const RESET_VALUE: u32 = 0x1111_1131;
}
