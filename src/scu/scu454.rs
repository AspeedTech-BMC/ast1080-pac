#[doc = "Register `SCU454` reader"]
pub type R = crate::R<Scu454Spec>;
#[doc = "Register `SCU454` writer"]
pub type W = crate::W<Scu454Spec>;
#[doc = "Field `SCUMUXIO168` reader - SCU_MUX_IO168"]
pub type Scumuxio168R = crate::FieldReader;
#[doc = "Field `SCUMUXIO168` writer - SCU_MUX_IO168"]
pub type Scumuxio168W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `Reserved7` reader - reserved"]
pub type Reserved7R = crate::BitReader;
#[doc = "Field `SCUMUXIO169` reader - SCU_MUX_IO169"]
pub type Scumuxio169R = crate::FieldReader;
#[doc = "Field `SCUMUXIO169` writer - SCU_MUX_IO169"]
pub type Scumuxio169W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `Reserved6` reader - reserved"]
pub type Reserved6R = crate::BitReader;
#[doc = "Field `SCUMUXIO170` reader - SCU_MUX_IO170"]
pub type Scumuxio170R = crate::FieldReader;
#[doc = "Field `SCUMUXIO170` writer - SCU_MUX_IO170"]
pub type Scumuxio170W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `Reserved5` reader - reserved"]
pub type Reserved5R = crate::BitReader;
#[doc = "Field `SCUMUXIO171` reader - SCU_MUX_IO171"]
pub type Scumuxio171R = crate::FieldReader;
#[doc = "Field `SCUMUXIO171` writer - SCU_MUX_IO171"]
pub type Scumuxio171W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `Reserved4` reader - reserved"]
pub type Reserved4R = crate::BitReader;
#[doc = "Field `SCUMUXIO172` reader - SCU_MUX_IO172"]
pub type Scumuxio172R = crate::FieldReader;
#[doc = "Field `SCUMUXIO172` writer - SCU_MUX_IO172"]
pub type Scumuxio172W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `Reserved3` reader - reserved"]
pub type Reserved3R = crate::BitReader;
#[doc = "Field `SCUMUXIO173` reader - SCU_MUX_IO173"]
pub type Scumuxio173R = crate::FieldReader;
#[doc = "Field `SCUMUXIO173` writer - SCU_MUX_IO173"]
pub type Scumuxio173W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `Reserved2` reader - reserved"]
pub type Reserved2R = crate::BitReader;
#[doc = "Field `SCUMUXIO174` reader - SCU_MUX_IO174"]
pub type Scumuxio174R = crate::FieldReader;
#[doc = "Field `SCUMUXIO174` writer - SCU_MUX_IO174"]
pub type Scumuxio174W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `Reserved1` reader - reserved"]
pub type Reserved1R = crate::BitReader;
#[doc = "Field `SCUMUXIO175` reader - SCU_MUX_IO175"]
pub type Scumuxio175R = crate::FieldReader;
#[doc = "Field `SCUMUXIO175` writer - SCU_MUX_IO175"]
pub type Scumuxio175W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2 - SCU_MUX_IO168"]
    #[inline(always)]
    pub fn scumuxio168(&self) -> Scumuxio168R {
        Scumuxio168R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3 - reserved"]
    #[inline(always)]
    pub fn reserved7(&self) -> Reserved7R {
        Reserved7R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:6 - SCU_MUX_IO169"]
    #[inline(always)]
    pub fn scumuxio169(&self) -> Scumuxio169R {
        Scumuxio169R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 7 - reserved"]
    #[inline(always)]
    pub fn reserved6(&self) -> Reserved6R {
        Reserved6R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:10 - SCU_MUX_IO170"]
    #[inline(always)]
    pub fn scumuxio170(&self) -> Scumuxio170R {
        Scumuxio170R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bit 11 - reserved"]
    #[inline(always)]
    pub fn reserved5(&self) -> Reserved5R {
        Reserved5R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:14 - SCU_MUX_IO171"]
    #[inline(always)]
    pub fn scumuxio171(&self) -> Scumuxio171R {
        Scumuxio171R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bit 15 - reserved"]
    #[inline(always)]
    pub fn reserved4(&self) -> Reserved4R {
        Reserved4R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:18 - SCU_MUX_IO172"]
    #[inline(always)]
    pub fn scumuxio172(&self) -> Scumuxio172R {
        Scumuxio172R::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bit 19 - reserved"]
    #[inline(always)]
    pub fn reserved3(&self) -> Reserved3R {
        Reserved3R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 20:22 - SCU_MUX_IO173"]
    #[inline(always)]
    pub fn scumuxio173(&self) -> Scumuxio173R {
        Scumuxio173R::new(((self.bits >> 20) & 7) as u8)
    }
    #[doc = "Bit 23 - reserved"]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:26 - SCU_MUX_IO174"]
    #[inline(always)]
    pub fn scumuxio174(&self) -> Scumuxio174R {
        Scumuxio174R::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bit 27 - reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bits 28:30 - SCU_MUX_IO175"]
    #[inline(always)]
    pub fn scumuxio175(&self) -> Scumuxio175R {
        Scumuxio175R::new(((self.bits >> 28) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - SCU_MUX_IO168"]
    #[inline(always)]
    pub fn scumuxio168(&mut self) -> Scumuxio168W<Scu454Spec> {
        Scumuxio168W::new(self, 0)
    }
    #[doc = "Bits 4:6 - SCU_MUX_IO169"]
    #[inline(always)]
    pub fn scumuxio169(&mut self) -> Scumuxio169W<Scu454Spec> {
        Scumuxio169W::new(self, 4)
    }
    #[doc = "Bits 8:10 - SCU_MUX_IO170"]
    #[inline(always)]
    pub fn scumuxio170(&mut self) -> Scumuxio170W<Scu454Spec> {
        Scumuxio170W::new(self, 8)
    }
    #[doc = "Bits 12:14 - SCU_MUX_IO171"]
    #[inline(always)]
    pub fn scumuxio171(&mut self) -> Scumuxio171W<Scu454Spec> {
        Scumuxio171W::new(self, 12)
    }
    #[doc = "Bits 16:18 - SCU_MUX_IO172"]
    #[inline(always)]
    pub fn scumuxio172(&mut self) -> Scumuxio172W<Scu454Spec> {
        Scumuxio172W::new(self, 16)
    }
    #[doc = "Bits 20:22 - SCU_MUX_IO173"]
    #[inline(always)]
    pub fn scumuxio173(&mut self) -> Scumuxio173W<Scu454Spec> {
        Scumuxio173W::new(self, 20)
    }
    #[doc = "Bits 24:26 - SCU_MUX_IO174"]
    #[inline(always)]
    pub fn scumuxio174(&mut self) -> Scumuxio174W<Scu454Spec> {
        Scumuxio174W::new(self, 24)
    }
    #[doc = "Bits 28:30 - SCU_MUX_IO175"]
    #[inline(always)]
    pub fn scumuxio175(&mut self) -> Scumuxio175W<Scu454Spec> {
        Scumuxio175W::new(self, 28)
    }
}
#[doc = "Multi-Function Pin Control \\#22\n\nYou can [`read`](crate::Reg::read) this register and get [`scu454::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu454::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scu454Spec;
impl crate::RegisterSpec for Scu454Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scu454::R`](R) reader structure"]
impl crate::Readable for Scu454Spec {}
#[doc = "`write(|w| ..)` method takes [`scu454::W`](W) writer structure"]
impl crate::Writable for Scu454Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCU454 to value 0x1111_1001"]
impl crate::Resettable for Scu454Spec {
    const RESET_VALUE: u32 = 0x1111_1001;
}
