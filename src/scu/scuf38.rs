#[doc = "Register `SCUF38` reader"]
pub type R = crate::R<Scuf38Spec>;
#[doc = "Register `SCUF38` writer"]
pub type W = crate::W<Scuf38Spec>;
#[doc = "Field `SCUREGRST700` reader - SCU_REG_RST_700"]
pub type Scuregrst700R = crate::BitReader;
#[doc = "Field `SCUREGRST700` writer - SCU_REG_RST_700"]
pub type Scuregrst700W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUREGRST704` reader - SCU_REG_RST_704"]
pub type Scuregrst704R = crate::BitReader;
#[doc = "Field `SCUREGRST704` writer - SCU_REG_RST_704"]
pub type Scuregrst704W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUREGRST708` reader - SCU_REG_RST_708"]
pub type Scuregrst708R = crate::BitReader;
#[doc = "Field `SCUREGRST708` writer - SCU_REG_RST_708"]
pub type Scuregrst708W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUREGRST70C` reader - SCU_REG_RST_70C"]
pub type Scuregrst70cR = crate::BitReader;
#[doc = "Field `SCUREGRST70C` writer - SCU_REG_RST_70C"]
pub type Scuregrst70cW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUREGRST710` reader - SCU_REG_RST_710"]
pub type Scuregrst710R = crate::BitReader;
#[doc = "Field `SCUREGRST710` writer - SCU_REG_RST_710"]
pub type Scuregrst710W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUREGRST714` reader - SCU_REG_RST_714"]
pub type Scuregrst714R = crate::BitReader;
#[doc = "Field `SCUREGRST714` writer - SCU_REG_RST_714"]
pub type Scuregrst714W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUREGRST718` reader - SCU_REG_RST_718"]
pub type Scuregrst718R = crate::BitReader;
#[doc = "Field `SCUREGRST718` writer - SCU_REG_RST_718"]
pub type Scuregrst718W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUREGRST71C` reader - SCU_REG_RST_71C"]
pub type Scuregrst71cR = crate::BitReader;
#[doc = "Field `SCUREGRST71C` writer - SCU_REG_RST_71C"]
pub type Scuregrst71cW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUREGRST720` reader - SCU_REG_RST_720"]
pub type Scuregrst720R = crate::BitReader;
#[doc = "Field `SCUREGRST720` writer - SCU_REG_RST_720"]
pub type Scuregrst720W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved1` reader - reserved"]
pub type Reserved1R = crate::FieldReader;
#[doc = "Field `SCUREGRST740` reader - SCU_REG_RST_740"]
pub type Scuregrst740R = crate::BitReader;
#[doc = "Field `SCUREGRST740` writer - SCU_REG_RST_740"]
pub type Scuregrst740W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUREGRST744` reader - SCU_REG_RST_744"]
pub type Scuregrst744R = crate::BitReader;
#[doc = "Field `SCUREGRST744` writer - SCU_REG_RST_744"]
pub type Scuregrst744W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUREGRST748` reader - SCU_REG_RST_748"]
pub type Scuregrst748R = crate::BitReader;
#[doc = "Field `SCUREGRST748` writer - SCU_REG_RST_748"]
pub type Scuregrst748W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUREGRST74C` reader - SCU_REG_RST_74C"]
pub type Scuregrst74cR = crate::BitReader;
#[doc = "Field `SCUREGRST74C` writer - SCU_REG_RST_74C"]
pub type Scuregrst74cW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUREGRST750` reader - SCU_REG_RST_750"]
pub type Scuregrst750R = crate::BitReader;
#[doc = "Field `SCUREGRST750` writer - SCU_REG_RST_750"]
pub type Scuregrst750W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUREGRST754` reader - SCU_REG_RST_754"]
pub type Scuregrst754R = crate::BitReader;
#[doc = "Field `SCUREGRST754` writer - SCU_REG_RST_754"]
pub type Scuregrst754W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUREGRST758` reader - SCU_REG_RST_758"]
pub type Scuregrst758R = crate::BitReader;
#[doc = "Field `SCUREGRST758` writer - SCU_REG_RST_758"]
pub type Scuregrst758W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - SCU_REG_RST_700"]
    #[inline(always)]
    pub fn scuregrst700(&self) -> Scuregrst700R {
        Scuregrst700R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - SCU_REG_RST_704"]
    #[inline(always)]
    pub fn scuregrst704(&self) -> Scuregrst704R {
        Scuregrst704R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - SCU_REG_RST_708"]
    #[inline(always)]
    pub fn scuregrst708(&self) -> Scuregrst708R {
        Scuregrst708R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - SCU_REG_RST_70C"]
    #[inline(always)]
    pub fn scuregrst70c(&self) -> Scuregrst70cR {
        Scuregrst70cR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - SCU_REG_RST_710"]
    #[inline(always)]
    pub fn scuregrst710(&self) -> Scuregrst710R {
        Scuregrst710R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - SCU_REG_RST_714"]
    #[inline(always)]
    pub fn scuregrst714(&self) -> Scuregrst714R {
        Scuregrst714R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - SCU_REG_RST_718"]
    #[inline(always)]
    pub fn scuregrst718(&self) -> Scuregrst718R {
        Scuregrst718R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - SCU_REG_RST_71C"]
    #[inline(always)]
    pub fn scuregrst71c(&self) -> Scuregrst71cR {
        Scuregrst71cR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - SCU_REG_RST_720"]
    #[inline(always)]
    pub fn scuregrst720(&self) -> Scuregrst720R {
        Scuregrst720R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:15 - reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 9) & 0x7f) as u8)
    }
    #[doc = "Bit 16 - SCU_REG_RST_740"]
    #[inline(always)]
    pub fn scuregrst740(&self) -> Scuregrst740R {
        Scuregrst740R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - SCU_REG_RST_744"]
    #[inline(always)]
    pub fn scuregrst744(&self) -> Scuregrst744R {
        Scuregrst744R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - SCU_REG_RST_748"]
    #[inline(always)]
    pub fn scuregrst748(&self) -> Scuregrst748R {
        Scuregrst748R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - SCU_REG_RST_74C"]
    #[inline(always)]
    pub fn scuregrst74c(&self) -> Scuregrst74cR {
        Scuregrst74cR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - SCU_REG_RST_750"]
    #[inline(always)]
    pub fn scuregrst750(&self) -> Scuregrst750R {
        Scuregrst750R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - SCU_REG_RST_754"]
    #[inline(always)]
    pub fn scuregrst754(&self) -> Scuregrst754R {
        Scuregrst754R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - SCU_REG_RST_758"]
    #[inline(always)]
    pub fn scuregrst758(&self) -> Scuregrst758R {
        Scuregrst758R::new(((self.bits >> 22) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SCU_REG_RST_700"]
    #[inline(always)]
    pub fn scuregrst700(&mut self) -> Scuregrst700W<Scuf38Spec> {
        Scuregrst700W::new(self, 0)
    }
    #[doc = "Bit 1 - SCU_REG_RST_704"]
    #[inline(always)]
    pub fn scuregrst704(&mut self) -> Scuregrst704W<Scuf38Spec> {
        Scuregrst704W::new(self, 1)
    }
    #[doc = "Bit 2 - SCU_REG_RST_708"]
    #[inline(always)]
    pub fn scuregrst708(&mut self) -> Scuregrst708W<Scuf38Spec> {
        Scuregrst708W::new(self, 2)
    }
    #[doc = "Bit 3 - SCU_REG_RST_70C"]
    #[inline(always)]
    pub fn scuregrst70c(&mut self) -> Scuregrst70cW<Scuf38Spec> {
        Scuregrst70cW::new(self, 3)
    }
    #[doc = "Bit 4 - SCU_REG_RST_710"]
    #[inline(always)]
    pub fn scuregrst710(&mut self) -> Scuregrst710W<Scuf38Spec> {
        Scuregrst710W::new(self, 4)
    }
    #[doc = "Bit 5 - SCU_REG_RST_714"]
    #[inline(always)]
    pub fn scuregrst714(&mut self) -> Scuregrst714W<Scuf38Spec> {
        Scuregrst714W::new(self, 5)
    }
    #[doc = "Bit 6 - SCU_REG_RST_718"]
    #[inline(always)]
    pub fn scuregrst718(&mut self) -> Scuregrst718W<Scuf38Spec> {
        Scuregrst718W::new(self, 6)
    }
    #[doc = "Bit 7 - SCU_REG_RST_71C"]
    #[inline(always)]
    pub fn scuregrst71c(&mut self) -> Scuregrst71cW<Scuf38Spec> {
        Scuregrst71cW::new(self, 7)
    }
    #[doc = "Bit 8 - SCU_REG_RST_720"]
    #[inline(always)]
    pub fn scuregrst720(&mut self) -> Scuregrst720W<Scuf38Spec> {
        Scuregrst720W::new(self, 8)
    }
    #[doc = "Bit 16 - SCU_REG_RST_740"]
    #[inline(always)]
    pub fn scuregrst740(&mut self) -> Scuregrst740W<Scuf38Spec> {
        Scuregrst740W::new(self, 16)
    }
    #[doc = "Bit 17 - SCU_REG_RST_744"]
    #[inline(always)]
    pub fn scuregrst744(&mut self) -> Scuregrst744W<Scuf38Spec> {
        Scuregrst744W::new(self, 17)
    }
    #[doc = "Bit 18 - SCU_REG_RST_748"]
    #[inline(always)]
    pub fn scuregrst748(&mut self) -> Scuregrst748W<Scuf38Spec> {
        Scuregrst748W::new(self, 18)
    }
    #[doc = "Bit 19 - SCU_REG_RST_74C"]
    #[inline(always)]
    pub fn scuregrst74c(&mut self) -> Scuregrst74cW<Scuf38Spec> {
        Scuregrst74cW::new(self, 19)
    }
    #[doc = "Bit 20 - SCU_REG_RST_750"]
    #[inline(always)]
    pub fn scuregrst750(&mut self) -> Scuregrst750W<Scuf38Spec> {
        Scuregrst750W::new(self, 20)
    }
    #[doc = "Bit 21 - SCU_REG_RST_754"]
    #[inline(always)]
    pub fn scuregrst754(&mut self) -> Scuregrst754W<Scuf38Spec> {
        Scuregrst754W::new(self, 21)
    }
    #[doc = "Bit 22 - SCU_REG_RST_758"]
    #[inline(always)]
    pub fn scuregrst758(&mut self) -> Scuregrst758W<Scuf38Spec> {
        Scuregrst758W::new(self, 22)
    }
}
#[doc = "Reset Control 15 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`scuf38::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scuf38::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scuf38Spec;
impl crate::RegisterSpec for Scuf38Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scuf38::R`](R) reader structure"]
impl crate::Readable for Scuf38Spec {}
#[doc = "`write(|w| ..)` method takes [`scuf38::W`](W) writer structure"]
impl crate::Writable for Scuf38Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCUF38 to value 0"]
impl crate::Resettable for Scuf38Spec {}
