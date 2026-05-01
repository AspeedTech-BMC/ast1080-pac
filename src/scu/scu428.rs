#[doc = "Register `SCU428` reader"]
pub type R = crate::R<Scu428Spec>;
#[doc = "Register `SCU428` writer"]
pub type W = crate::W<Scu428Spec>;
#[doc = "Field `SCUMUXIO080` reader - SCU_MUX_IO080"]
pub type Scumuxio080R = crate::FieldReader;
#[doc = "Field `SCUMUXIO080` writer - SCU_MUX_IO080"]
pub type Scumuxio080W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `Reserved7` reader - reserved"]
pub type Reserved7R = crate::BitReader;
#[doc = "Field `SCUMUXIO081` reader - SCU_MUX_IO081"]
pub type Scumuxio081R = crate::FieldReader;
#[doc = "Field `SCUMUXIO081` writer - SCU_MUX_IO081"]
pub type Scumuxio081W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `Reserved6` reader - reserved"]
pub type Reserved6R = crate::BitReader;
#[doc = "Field `SCUMUXIO082` reader - SCU_MUX_IO082"]
pub type Scumuxio082R = crate::FieldReader;
#[doc = "Field `SCUMUXIO082` writer - SCU_MUX_IO082"]
pub type Scumuxio082W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `Reserved5` reader - reserved"]
pub type Reserved5R = crate::BitReader;
#[doc = "Field `SCUMUXIO083` reader - SCU_MUX_IO083"]
pub type Scumuxio083R = crate::FieldReader;
#[doc = "Field `SCUMUXIO083` writer - SCU_MUX_IO083"]
pub type Scumuxio083W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `Reserved4` reader - reserved"]
pub type Reserved4R = crate::BitReader;
#[doc = "Field `SCUMUXIO084` reader - SCU_MUX_IO084"]
pub type Scumuxio084R = crate::FieldReader;
#[doc = "Field `SCUMUXIO084` writer - SCU_MUX_IO084"]
pub type Scumuxio084W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `Reserved3` reader - reserved"]
pub type Reserved3R = crate::BitReader;
#[doc = "Field `SCUMUXIO085` reader - SCU_MUX_IO085"]
pub type Scumuxio085R = crate::FieldReader;
#[doc = "Field `SCUMUXIO085` writer - SCU_MUX_IO085"]
pub type Scumuxio085W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `Reserved2` reader - reserved"]
pub type Reserved2R = crate::BitReader;
#[doc = "Field `SCUMUXIO086` reader - SCU_MUX_IO086"]
pub type Scumuxio086R = crate::FieldReader;
#[doc = "Field `SCUMUXIO086` writer - SCU_MUX_IO086"]
pub type Scumuxio086W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `Reserved1` reader - reserved"]
pub type Reserved1R = crate::BitReader;
#[doc = "Field `SCUMUXIO087` reader - SCU_MUX_IO087"]
pub type Scumuxio087R = crate::FieldReader;
#[doc = "Field `SCUMUXIO087` writer - SCU_MUX_IO087"]
pub type Scumuxio087W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2 - SCU_MUX_IO080"]
    #[inline(always)]
    pub fn scumuxio080(&self) -> Scumuxio080R {
        Scumuxio080R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3 - reserved"]
    #[inline(always)]
    pub fn reserved7(&self) -> Reserved7R {
        Reserved7R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:6 - SCU_MUX_IO081"]
    #[inline(always)]
    pub fn scumuxio081(&self) -> Scumuxio081R {
        Scumuxio081R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 7 - reserved"]
    #[inline(always)]
    pub fn reserved6(&self) -> Reserved6R {
        Reserved6R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:10 - SCU_MUX_IO082"]
    #[inline(always)]
    pub fn scumuxio082(&self) -> Scumuxio082R {
        Scumuxio082R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bit 11 - reserved"]
    #[inline(always)]
    pub fn reserved5(&self) -> Reserved5R {
        Reserved5R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:14 - SCU_MUX_IO083"]
    #[inline(always)]
    pub fn scumuxio083(&self) -> Scumuxio083R {
        Scumuxio083R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bit 15 - reserved"]
    #[inline(always)]
    pub fn reserved4(&self) -> Reserved4R {
        Reserved4R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:18 - SCU_MUX_IO084"]
    #[inline(always)]
    pub fn scumuxio084(&self) -> Scumuxio084R {
        Scumuxio084R::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bit 19 - reserved"]
    #[inline(always)]
    pub fn reserved3(&self) -> Reserved3R {
        Reserved3R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 20:22 - SCU_MUX_IO085"]
    #[inline(always)]
    pub fn scumuxio085(&self) -> Scumuxio085R {
        Scumuxio085R::new(((self.bits >> 20) & 7) as u8)
    }
    #[doc = "Bit 23 - reserved"]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:26 - SCU_MUX_IO086"]
    #[inline(always)]
    pub fn scumuxio086(&self) -> Scumuxio086R {
        Scumuxio086R::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bit 27 - reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bits 28:30 - SCU_MUX_IO087"]
    #[inline(always)]
    pub fn scumuxio087(&self) -> Scumuxio087R {
        Scumuxio087R::new(((self.bits >> 28) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - SCU_MUX_IO080"]
    #[inline(always)]
    pub fn scumuxio080(&mut self) -> Scumuxio080W<Scu428Spec> {
        Scumuxio080W::new(self, 0)
    }
    #[doc = "Bits 4:6 - SCU_MUX_IO081"]
    #[inline(always)]
    pub fn scumuxio081(&mut self) -> Scumuxio081W<Scu428Spec> {
        Scumuxio081W::new(self, 4)
    }
    #[doc = "Bits 8:10 - SCU_MUX_IO082"]
    #[inline(always)]
    pub fn scumuxio082(&mut self) -> Scumuxio082W<Scu428Spec> {
        Scumuxio082W::new(self, 8)
    }
    #[doc = "Bits 12:14 - SCU_MUX_IO083"]
    #[inline(always)]
    pub fn scumuxio083(&mut self) -> Scumuxio083W<Scu428Spec> {
        Scumuxio083W::new(self, 12)
    }
    #[doc = "Bits 16:18 - SCU_MUX_IO084"]
    #[inline(always)]
    pub fn scumuxio084(&mut self) -> Scumuxio084W<Scu428Spec> {
        Scumuxio084W::new(self, 16)
    }
    #[doc = "Bits 20:22 - SCU_MUX_IO085"]
    #[inline(always)]
    pub fn scumuxio085(&mut self) -> Scumuxio085W<Scu428Spec> {
        Scumuxio085W::new(self, 20)
    }
    #[doc = "Bits 24:26 - SCU_MUX_IO086"]
    #[inline(always)]
    pub fn scumuxio086(&mut self) -> Scumuxio086W<Scu428Spec> {
        Scumuxio086W::new(self, 24)
    }
    #[doc = "Bits 28:30 - SCU_MUX_IO087"]
    #[inline(always)]
    pub fn scumuxio087(&mut self) -> Scumuxio087W<Scu428Spec> {
        Scumuxio087W::new(self, 28)
    }
}
#[doc = "Multi-Function Pin Control \\#11\n\nYou can [`read`](crate::Reg::read) this register and get [`scu428::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu428::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scu428Spec;
impl crate::RegisterSpec for Scu428Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scu428::R`](R) reader structure"]
impl crate::Readable for Scu428Spec {}
#[doc = "`write(|w| ..)` method takes [`scu428::W`](W) writer structure"]
impl crate::Writable for Scu428Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCU428 to value 0"]
impl crate::Resettable for Scu428Spec {}
