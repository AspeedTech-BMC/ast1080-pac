#[doc = "Register `SCU408` reader"]
pub type R = crate::R<Scu408Spec>;
#[doc = "Register `SCU408` writer"]
pub type W = crate::W<Scu408Spec>;
#[doc = "Field `SCUMUXIO016` reader - SCU_MUX_IO016"]
pub type Scumuxio016R = crate::FieldReader;
#[doc = "Field `SCUMUXIO016` writer - SCU_MUX_IO016"]
pub type Scumuxio016W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `Reserved7` reader - reserved"]
pub type Reserved7R = crate::BitReader;
#[doc = "Field `SCUMUXIO017` reader - SCU_MUX_IO017"]
pub type Scumuxio017R = crate::FieldReader;
#[doc = "Field `SCUMUXIO017` writer - SCU_MUX_IO017"]
pub type Scumuxio017W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `Reserved6` reader - reserved"]
pub type Reserved6R = crate::BitReader;
#[doc = "Field `SCUMUXIO018` reader - SCU_MUX_IO018"]
pub type Scumuxio018R = crate::FieldReader;
#[doc = "Field `SCUMUXIO018` writer - SCU_MUX_IO018"]
pub type Scumuxio018W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `Reserved5` reader - reserved"]
pub type Reserved5R = crate::BitReader;
#[doc = "Field `SCUMUXIO019` reader - SCU_MUX_IO019"]
pub type Scumuxio019R = crate::FieldReader;
#[doc = "Field `SCUMUXIO019` writer - SCU_MUX_IO019"]
pub type Scumuxio019W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `Reserved4` reader - reserved"]
pub type Reserved4R = crate::BitReader;
#[doc = "Field `SCUMUXIO020` reader - SCU_MUX_IO020"]
pub type Scumuxio020R = crate::FieldReader;
#[doc = "Field `SCUMUXIO020` writer - SCU_MUX_IO020"]
pub type Scumuxio020W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `Reserved3` reader - reserved"]
pub type Reserved3R = crate::BitReader;
#[doc = "Field `SCUMUXIO021` reader - SCU_MUX_IO021"]
pub type Scumuxio021R = crate::FieldReader;
#[doc = "Field `SCUMUXIO021` writer - SCU_MUX_IO021"]
pub type Scumuxio021W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `Reserved2` reader - reserved"]
pub type Reserved2R = crate::BitReader;
#[doc = "Field `SCUMUXIO022` reader - SCU_MUX_IO022"]
pub type Scumuxio022R = crate::FieldReader;
#[doc = "Field `SCUMUXIO022` writer - SCU_MUX_IO022"]
pub type Scumuxio022W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `Reserved1` reader - reserved"]
pub type Reserved1R = crate::BitReader;
#[doc = "Field `SCUMUXIO023` reader - SCU_MUX_IO023"]
pub type Scumuxio023R = crate::FieldReader;
#[doc = "Field `SCUMUXIO023` writer - SCU_MUX_IO023"]
pub type Scumuxio023W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2 - SCU_MUX_IO016"]
    #[inline(always)]
    pub fn scumuxio016(&self) -> Scumuxio016R {
        Scumuxio016R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3 - reserved"]
    #[inline(always)]
    pub fn reserved7(&self) -> Reserved7R {
        Reserved7R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:6 - SCU_MUX_IO017"]
    #[inline(always)]
    pub fn scumuxio017(&self) -> Scumuxio017R {
        Scumuxio017R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 7 - reserved"]
    #[inline(always)]
    pub fn reserved6(&self) -> Reserved6R {
        Reserved6R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:10 - SCU_MUX_IO018"]
    #[inline(always)]
    pub fn scumuxio018(&self) -> Scumuxio018R {
        Scumuxio018R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bit 11 - reserved"]
    #[inline(always)]
    pub fn reserved5(&self) -> Reserved5R {
        Reserved5R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:14 - SCU_MUX_IO019"]
    #[inline(always)]
    pub fn scumuxio019(&self) -> Scumuxio019R {
        Scumuxio019R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bit 15 - reserved"]
    #[inline(always)]
    pub fn reserved4(&self) -> Reserved4R {
        Reserved4R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:18 - SCU_MUX_IO020"]
    #[inline(always)]
    pub fn scumuxio020(&self) -> Scumuxio020R {
        Scumuxio020R::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bit 19 - reserved"]
    #[inline(always)]
    pub fn reserved3(&self) -> Reserved3R {
        Reserved3R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 20:22 - SCU_MUX_IO021"]
    #[inline(always)]
    pub fn scumuxio021(&self) -> Scumuxio021R {
        Scumuxio021R::new(((self.bits >> 20) & 7) as u8)
    }
    #[doc = "Bit 23 - reserved"]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:26 - SCU_MUX_IO022"]
    #[inline(always)]
    pub fn scumuxio022(&self) -> Scumuxio022R {
        Scumuxio022R::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bit 27 - reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bits 28:30 - SCU_MUX_IO023"]
    #[inline(always)]
    pub fn scumuxio023(&self) -> Scumuxio023R {
        Scumuxio023R::new(((self.bits >> 28) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - SCU_MUX_IO016"]
    #[inline(always)]
    pub fn scumuxio016(&mut self) -> Scumuxio016W<Scu408Spec> {
        Scumuxio016W::new(self, 0)
    }
    #[doc = "Bits 4:6 - SCU_MUX_IO017"]
    #[inline(always)]
    pub fn scumuxio017(&mut self) -> Scumuxio017W<Scu408Spec> {
        Scumuxio017W::new(self, 4)
    }
    #[doc = "Bits 8:10 - SCU_MUX_IO018"]
    #[inline(always)]
    pub fn scumuxio018(&mut self) -> Scumuxio018W<Scu408Spec> {
        Scumuxio018W::new(self, 8)
    }
    #[doc = "Bits 12:14 - SCU_MUX_IO019"]
    #[inline(always)]
    pub fn scumuxio019(&mut self) -> Scumuxio019W<Scu408Spec> {
        Scumuxio019W::new(self, 12)
    }
    #[doc = "Bits 16:18 - SCU_MUX_IO020"]
    #[inline(always)]
    pub fn scumuxio020(&mut self) -> Scumuxio020W<Scu408Spec> {
        Scumuxio020W::new(self, 16)
    }
    #[doc = "Bits 20:22 - SCU_MUX_IO021"]
    #[inline(always)]
    pub fn scumuxio021(&mut self) -> Scumuxio021W<Scu408Spec> {
        Scumuxio021W::new(self, 20)
    }
    #[doc = "Bits 24:26 - SCU_MUX_IO022"]
    #[inline(always)]
    pub fn scumuxio022(&mut self) -> Scumuxio022W<Scu408Spec> {
        Scumuxio022W::new(self, 24)
    }
    #[doc = "Bits 28:30 - SCU_MUX_IO023"]
    #[inline(always)]
    pub fn scumuxio023(&mut self) -> Scumuxio023W<Scu408Spec> {
        Scumuxio023W::new(self, 28)
    }
}
#[doc = "Multi-Function Pin Control \\#3\n\nYou can [`read`](crate::Reg::read) this register and get [`scu408::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu408::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scu408Spec;
impl crate::RegisterSpec for Scu408Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scu408::R`](R) reader structure"]
impl crate::Readable for Scu408Spec {}
#[doc = "`write(|w| ..)` method takes [`scu408::W`](W) writer structure"]
impl crate::Writable for Scu408Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCU408 to value 0"]
impl crate::Resettable for Scu408Spec {}
