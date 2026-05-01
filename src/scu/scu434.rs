#[doc = "Register `SCU434` reader"]
pub type R = crate::R<Scu434Spec>;
#[doc = "Register `SCU434` writer"]
pub type W = crate::W<Scu434Spec>;
#[doc = "Field `SCUMUXIO104` reader - SCU_MUX_IO104"]
pub type Scumuxio104R = crate::FieldReader;
#[doc = "Field `SCUMUXIO104` writer - SCU_MUX_IO104"]
pub type Scumuxio104W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `Reserved7` reader - reserved"]
pub type Reserved7R = crate::BitReader;
#[doc = "Field `SCUMUXIO105` reader - SCU_MUX_IO105"]
pub type Scumuxio105R = crate::FieldReader;
#[doc = "Field `SCUMUXIO105` writer - SCU_MUX_IO105"]
pub type Scumuxio105W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `Reserved6` reader - reserved"]
pub type Reserved6R = crate::BitReader;
#[doc = "Field `SCUMUXIO106` reader - SCU_MUX_IO106"]
pub type Scumuxio106R = crate::FieldReader;
#[doc = "Field `SCUMUXIO106` writer - SCU_MUX_IO106"]
pub type Scumuxio106W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `Reserved5` reader - reserved"]
pub type Reserved5R = crate::BitReader;
#[doc = "Field `SCUMUXIO107` reader - SCU_MUX_IO107"]
pub type Scumuxio107R = crate::FieldReader;
#[doc = "Field `SCUMUXIO107` writer - SCU_MUX_IO107"]
pub type Scumuxio107W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `Reserved4` reader - reserved"]
pub type Reserved4R = crate::BitReader;
#[doc = "Field `SCUMUXIO108` reader - SCU_MUX_IO108"]
pub type Scumuxio108R = crate::FieldReader;
#[doc = "Field `SCUMUXIO108` writer - SCU_MUX_IO108"]
pub type Scumuxio108W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `Reserved3` reader - reserved"]
pub type Reserved3R = crate::BitReader;
#[doc = "Field `SCUMUXIO109` reader - SCU_MUX_IO109"]
pub type Scumuxio109R = crate::FieldReader;
#[doc = "Field `SCUMUXIO109` writer - SCU_MUX_IO109"]
pub type Scumuxio109W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `Reserved2` reader - reserved"]
pub type Reserved2R = crate::BitReader;
#[doc = "Field `SCUMUXIO110` reader - SCU_MUX_IO110"]
pub type Scumuxio110R = crate::FieldReader;
#[doc = "Field `SCUMUXIO110` writer - SCU_MUX_IO110"]
pub type Scumuxio110W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `Reserved1` reader - reserved"]
pub type Reserved1R = crate::BitReader;
#[doc = "Field `SCUMUXIO111` reader - SCU_MUX_IO111"]
pub type Scumuxio111R = crate::FieldReader;
#[doc = "Field `SCUMUXIO111` writer - SCU_MUX_IO111"]
pub type Scumuxio111W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2 - SCU_MUX_IO104"]
    #[inline(always)]
    pub fn scumuxio104(&self) -> Scumuxio104R {
        Scumuxio104R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3 - reserved"]
    #[inline(always)]
    pub fn reserved7(&self) -> Reserved7R {
        Reserved7R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:6 - SCU_MUX_IO105"]
    #[inline(always)]
    pub fn scumuxio105(&self) -> Scumuxio105R {
        Scumuxio105R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 7 - reserved"]
    #[inline(always)]
    pub fn reserved6(&self) -> Reserved6R {
        Reserved6R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:10 - SCU_MUX_IO106"]
    #[inline(always)]
    pub fn scumuxio106(&self) -> Scumuxio106R {
        Scumuxio106R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bit 11 - reserved"]
    #[inline(always)]
    pub fn reserved5(&self) -> Reserved5R {
        Reserved5R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:14 - SCU_MUX_IO107"]
    #[inline(always)]
    pub fn scumuxio107(&self) -> Scumuxio107R {
        Scumuxio107R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bit 15 - reserved"]
    #[inline(always)]
    pub fn reserved4(&self) -> Reserved4R {
        Reserved4R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:18 - SCU_MUX_IO108"]
    #[inline(always)]
    pub fn scumuxio108(&self) -> Scumuxio108R {
        Scumuxio108R::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bit 19 - reserved"]
    #[inline(always)]
    pub fn reserved3(&self) -> Reserved3R {
        Reserved3R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 20:22 - SCU_MUX_IO109"]
    #[inline(always)]
    pub fn scumuxio109(&self) -> Scumuxio109R {
        Scumuxio109R::new(((self.bits >> 20) & 7) as u8)
    }
    #[doc = "Bit 23 - reserved"]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:26 - SCU_MUX_IO110"]
    #[inline(always)]
    pub fn scumuxio110(&self) -> Scumuxio110R {
        Scumuxio110R::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bit 27 - reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bits 28:30 - SCU_MUX_IO111"]
    #[inline(always)]
    pub fn scumuxio111(&self) -> Scumuxio111R {
        Scumuxio111R::new(((self.bits >> 28) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - SCU_MUX_IO104"]
    #[inline(always)]
    pub fn scumuxio104(&mut self) -> Scumuxio104W<Scu434Spec> {
        Scumuxio104W::new(self, 0)
    }
    #[doc = "Bits 4:6 - SCU_MUX_IO105"]
    #[inline(always)]
    pub fn scumuxio105(&mut self) -> Scumuxio105W<Scu434Spec> {
        Scumuxio105W::new(self, 4)
    }
    #[doc = "Bits 8:10 - SCU_MUX_IO106"]
    #[inline(always)]
    pub fn scumuxio106(&mut self) -> Scumuxio106W<Scu434Spec> {
        Scumuxio106W::new(self, 8)
    }
    #[doc = "Bits 12:14 - SCU_MUX_IO107"]
    #[inline(always)]
    pub fn scumuxio107(&mut self) -> Scumuxio107W<Scu434Spec> {
        Scumuxio107W::new(self, 12)
    }
    #[doc = "Bits 16:18 - SCU_MUX_IO108"]
    #[inline(always)]
    pub fn scumuxio108(&mut self) -> Scumuxio108W<Scu434Spec> {
        Scumuxio108W::new(self, 16)
    }
    #[doc = "Bits 20:22 - SCU_MUX_IO109"]
    #[inline(always)]
    pub fn scumuxio109(&mut self) -> Scumuxio109W<Scu434Spec> {
        Scumuxio109W::new(self, 20)
    }
    #[doc = "Bits 24:26 - SCU_MUX_IO110"]
    #[inline(always)]
    pub fn scumuxio110(&mut self) -> Scumuxio110W<Scu434Spec> {
        Scumuxio110W::new(self, 24)
    }
    #[doc = "Bits 28:30 - SCU_MUX_IO111"]
    #[inline(always)]
    pub fn scumuxio111(&mut self) -> Scumuxio111W<Scu434Spec> {
        Scumuxio111W::new(self, 28)
    }
}
#[doc = "Multi-Function Pin Control \\#14\n\nYou can [`read`](crate::Reg::read) this register and get [`scu434::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu434::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scu434Spec;
impl crate::RegisterSpec for Scu434Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scu434::R`](R) reader structure"]
impl crate::Readable for Scu434Spec {}
#[doc = "`write(|w| ..)` method takes [`scu434::W`](W) writer structure"]
impl crate::Writable for Scu434Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCU434 to value 0"]
impl crate::Resettable for Scu434Spec {}
