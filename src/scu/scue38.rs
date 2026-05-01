#[doc = "Register `SCUE38` reader"]
pub type R = crate::R<Scue38Spec>;
#[doc = "Register `SCUE38` writer"]
pub type W = crate::W<Scue38Spec>;
#[doc = "Field `SCUREGLOCK700` reader - SCU_REG_LOCK_700"]
pub type Scureglock700R = crate::BitReader;
#[doc = "Field `SCUREGLOCK700` writer - SCU_REG_LOCK_700"]
pub type Scureglock700W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUREGLOCK704` reader - SCU_REG_LOCK_704"]
pub type Scureglock704R = crate::BitReader;
#[doc = "Field `SCUREGLOCK704` writer - SCU_REG_LOCK_704"]
pub type Scureglock704W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUREGLOCK708` reader - SCU_REG_LOCK_708"]
pub type Scureglock708R = crate::BitReader;
#[doc = "Field `SCUREGLOCK708` writer - SCU_REG_LOCK_708"]
pub type Scureglock708W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUREGLOCK70C` reader - SCU_REG_LOCK_70C"]
pub type Scureglock70cR = crate::BitReader;
#[doc = "Field `SCUREGLOCK70C` writer - SCU_REG_LOCK_70C"]
pub type Scureglock70cW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUREGLOCK710` reader - SCU_REG_LOCK_710"]
pub type Scureglock710R = crate::BitReader;
#[doc = "Field `SCUREGLOCK710` writer - SCU_REG_LOCK_710"]
pub type Scureglock710W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUREGLOCK714` reader - SCU_REG_LOCK_714"]
pub type Scureglock714R = crate::BitReader;
#[doc = "Field `SCUREGLOCK714` writer - SCU_REG_LOCK_714"]
pub type Scureglock714W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUREGLOCK718` reader - SCU_REG_LOCK_718"]
pub type Scureglock718R = crate::BitReader;
#[doc = "Field `SCUREGLOCK718` writer - SCU_REG_LOCK_718"]
pub type Scureglock718W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved1` reader - reserved"]
pub type Reserved1R = crate::FieldReader<u16>;
#[doc = "Field `SCUREGLOCK740` reader - SCU_REG_LOCK_740"]
pub type Scureglock740R = crate::BitReader;
#[doc = "Field `SCUREGLOCK740` writer - SCU_REG_LOCK_740"]
pub type Scureglock740W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUREGLOCK744` reader - SCU_REG_LOCK_744"]
pub type Scureglock744R = crate::BitReader;
#[doc = "Field `SCUREGLOCK744` writer - SCU_REG_LOCK_744"]
pub type Scureglock744W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUREGLOCK748` reader - SCU_REG_LOCK_748"]
pub type Scureglock748R = crate::BitReader;
#[doc = "Field `SCUREGLOCK748` writer - SCU_REG_LOCK_748"]
pub type Scureglock748W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUREGLOCK74C` reader - SCU_REG_LOCK_74C"]
pub type Scureglock74cR = crate::BitReader;
#[doc = "Field `SCUREGLOCK74C` writer - SCU_REG_LOCK_74C"]
pub type Scureglock74cW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUREGLOCK750` reader - SCU_REG_LOCK_750"]
pub type Scureglock750R = crate::BitReader;
#[doc = "Field `SCUREGLOCK750` writer - SCU_REG_LOCK_750"]
pub type Scureglock750W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUREGLOCK754` reader - SCU_REG_LOCK_754"]
pub type Scureglock754R = crate::BitReader;
#[doc = "Field `SCUREGLOCK754` writer - SCU_REG_LOCK_754"]
pub type Scureglock754W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUREGLOCK758` reader - SCU_REG_LOCK_758"]
pub type Scureglock758R = crate::BitReader;
#[doc = "Field `SCUREGLOCK758` writer - SCU_REG_LOCK_758"]
pub type Scureglock758W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - SCU_REG_LOCK_700"]
    #[inline(always)]
    pub fn scureglock700(&self) -> Scureglock700R {
        Scureglock700R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - SCU_REG_LOCK_704"]
    #[inline(always)]
    pub fn scureglock704(&self) -> Scureglock704R {
        Scureglock704R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - SCU_REG_LOCK_708"]
    #[inline(always)]
    pub fn scureglock708(&self) -> Scureglock708R {
        Scureglock708R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - SCU_REG_LOCK_70C"]
    #[inline(always)]
    pub fn scureglock70c(&self) -> Scureglock70cR {
        Scureglock70cR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - SCU_REG_LOCK_710"]
    #[inline(always)]
    pub fn scureglock710(&self) -> Scureglock710R {
        Scureglock710R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - SCU_REG_LOCK_714"]
    #[inline(always)]
    pub fn scureglock714(&self) -> Scureglock714R {
        Scureglock714R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - SCU_REG_LOCK_718"]
    #[inline(always)]
    pub fn scureglock718(&self) -> Scureglock718R {
        Scureglock718R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 7:15 - reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 7) & 0x01ff) as u16)
    }
    #[doc = "Bit 16 - SCU_REG_LOCK_740"]
    #[inline(always)]
    pub fn scureglock740(&self) -> Scureglock740R {
        Scureglock740R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - SCU_REG_LOCK_744"]
    #[inline(always)]
    pub fn scureglock744(&self) -> Scureglock744R {
        Scureglock744R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - SCU_REG_LOCK_748"]
    #[inline(always)]
    pub fn scureglock748(&self) -> Scureglock748R {
        Scureglock748R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - SCU_REG_LOCK_74C"]
    #[inline(always)]
    pub fn scureglock74c(&self) -> Scureglock74cR {
        Scureglock74cR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - SCU_REG_LOCK_750"]
    #[inline(always)]
    pub fn scureglock750(&self) -> Scureglock750R {
        Scureglock750R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - SCU_REG_LOCK_754"]
    #[inline(always)]
    pub fn scureglock754(&self) -> Scureglock754R {
        Scureglock754R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - SCU_REG_LOCK_758"]
    #[inline(always)]
    pub fn scureglock758(&self) -> Scureglock758R {
        Scureglock758R::new(((self.bits >> 22) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SCU_REG_LOCK_700"]
    #[inline(always)]
    pub fn scureglock700(&mut self) -> Scureglock700W<Scue38Spec> {
        Scureglock700W::new(self, 0)
    }
    #[doc = "Bit 1 - SCU_REG_LOCK_704"]
    #[inline(always)]
    pub fn scureglock704(&mut self) -> Scureglock704W<Scue38Spec> {
        Scureglock704W::new(self, 1)
    }
    #[doc = "Bit 2 - SCU_REG_LOCK_708"]
    #[inline(always)]
    pub fn scureglock708(&mut self) -> Scureglock708W<Scue38Spec> {
        Scureglock708W::new(self, 2)
    }
    #[doc = "Bit 3 - SCU_REG_LOCK_70C"]
    #[inline(always)]
    pub fn scureglock70c(&mut self) -> Scureglock70cW<Scue38Spec> {
        Scureglock70cW::new(self, 3)
    }
    #[doc = "Bit 4 - SCU_REG_LOCK_710"]
    #[inline(always)]
    pub fn scureglock710(&mut self) -> Scureglock710W<Scue38Spec> {
        Scureglock710W::new(self, 4)
    }
    #[doc = "Bit 5 - SCU_REG_LOCK_714"]
    #[inline(always)]
    pub fn scureglock714(&mut self) -> Scureglock714W<Scue38Spec> {
        Scureglock714W::new(self, 5)
    }
    #[doc = "Bit 6 - SCU_REG_LOCK_718"]
    #[inline(always)]
    pub fn scureglock718(&mut self) -> Scureglock718W<Scue38Spec> {
        Scureglock718W::new(self, 6)
    }
    #[doc = "Bit 16 - SCU_REG_LOCK_740"]
    #[inline(always)]
    pub fn scureglock740(&mut self) -> Scureglock740W<Scue38Spec> {
        Scureglock740W::new(self, 16)
    }
    #[doc = "Bit 17 - SCU_REG_LOCK_744"]
    #[inline(always)]
    pub fn scureglock744(&mut self) -> Scureglock744W<Scue38Spec> {
        Scureglock744W::new(self, 17)
    }
    #[doc = "Bit 18 - SCU_REG_LOCK_748"]
    #[inline(always)]
    pub fn scureglock748(&mut self) -> Scureglock748W<Scue38Spec> {
        Scureglock748W::new(self, 18)
    }
    #[doc = "Bit 19 - SCU_REG_LOCK_74C"]
    #[inline(always)]
    pub fn scureglock74c(&mut self) -> Scureglock74cW<Scue38Spec> {
        Scureglock74cW::new(self, 19)
    }
    #[doc = "Bit 20 - SCU_REG_LOCK_750"]
    #[inline(always)]
    pub fn scureglock750(&mut self) -> Scureglock750W<Scue38Spec> {
        Scureglock750W::new(self, 20)
    }
    #[doc = "Bit 21 - SCU_REG_LOCK_754"]
    #[inline(always)]
    pub fn scureglock754(&mut self) -> Scureglock754W<Scue38Spec> {
        Scureglock754W::new(self, 21)
    }
    #[doc = "Bit 22 - SCU_REG_LOCK_758"]
    #[inline(always)]
    pub fn scureglock758(&mut self) -> Scureglock758W<Scue38Spec> {
        Scureglock758W::new(self, 22)
    }
}
#[doc = "Write Protection 15 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`scue38::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scue38::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scue38Spec;
impl crate::RegisterSpec for Scue38Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scue38::R`](R) reader structure"]
impl crate::Readable for Scue38Spec {}
#[doc = "`write(|w| ..)` method takes [`scue38::W`](W) writer structure"]
impl crate::Writable for Scue38Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCUE38 to value 0"]
impl crate::Resettable for Scue38Spec {}
