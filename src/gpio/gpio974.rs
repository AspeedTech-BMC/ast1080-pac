#[doc = "Register `GPIO974` reader"]
pub type R = crate::R<Gpio974Spec>;
#[doc = "Register `GPIO974` writer"]
pub type W = crate::W<Gpio974Spec>;
#[doc = "Field `GPIO100ReadPrivilegeOfMaster` reader - GPIO100 Read Privilege of Master"]
pub type Gpio100readPrivilegeOfMasterR = crate::FieldReader;
#[doc = "Field `GPIO100ReadPrivilegeOfMaster` writer - GPIO100 Read Privilege of Master"]
pub type Gpio100readPrivilegeOfMasterW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `GPIO101ReadPrivilegeOfMaster` reader - GPIO101 Read Privilege of Master"]
pub type Gpio101readPrivilegeOfMasterR = crate::FieldReader;
#[doc = "Field `GPIO101ReadPrivilegeOfMaster` writer - GPIO101 Read Privilege of Master"]
pub type Gpio101readPrivilegeOfMasterW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `GPIO102ReadPrivilegeOfMaster` reader - GPIO102 Read Privilege of Master"]
pub type Gpio102readPrivilegeOfMasterR = crate::FieldReader;
#[doc = "Field `GPIO102ReadPrivilegeOfMaster` writer - GPIO102 Read Privilege of Master"]
pub type Gpio102readPrivilegeOfMasterW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `GPIO103ReadPrivilegeOfMaster` reader - GPIO103 Read Privilege of Master"]
pub type Gpio103readPrivilegeOfMasterR = crate::FieldReader;
#[doc = "Field `GPIO103ReadPrivilegeOfMaster` writer - GPIO103 Read Privilege of Master"]
pub type Gpio103readPrivilegeOfMasterW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - GPIO100 Read Privilege of Master"]
    #[inline(always)]
    pub fn gpio100read_privilege_of_master(&self) -> Gpio100readPrivilegeOfMasterR {
        Gpio100readPrivilegeOfMasterR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - GPIO101 Read Privilege of Master"]
    #[inline(always)]
    pub fn gpio101read_privilege_of_master(&self) -> Gpio101readPrivilegeOfMasterR {
        Gpio101readPrivilegeOfMasterR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - GPIO102 Read Privilege of Master"]
    #[inline(always)]
    pub fn gpio102read_privilege_of_master(&self) -> Gpio102readPrivilegeOfMasterR {
        Gpio102readPrivilegeOfMasterR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - GPIO103 Read Privilege of Master"]
    #[inline(always)]
    pub fn gpio103read_privilege_of_master(&self) -> Gpio103readPrivilegeOfMasterR {
        Gpio103readPrivilegeOfMasterR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - GPIO100 Read Privilege of Master"]
    #[inline(always)]
    pub fn gpio100read_privilege_of_master(
        &mut self,
    ) -> Gpio100readPrivilegeOfMasterW<Gpio974Spec> {
        Gpio100readPrivilegeOfMasterW::new(self, 0)
    }
    #[doc = "Bits 8:15 - GPIO101 Read Privilege of Master"]
    #[inline(always)]
    pub fn gpio101read_privilege_of_master(
        &mut self,
    ) -> Gpio101readPrivilegeOfMasterW<Gpio974Spec> {
        Gpio101readPrivilegeOfMasterW::new(self, 8)
    }
    #[doc = "Bits 16:23 - GPIO102 Read Privilege of Master"]
    #[inline(always)]
    pub fn gpio102read_privilege_of_master(
        &mut self,
    ) -> Gpio102readPrivilegeOfMasterW<Gpio974Spec> {
        Gpio102readPrivilegeOfMasterW::new(self, 16)
    }
    #[doc = "Bits 24:31 - GPIO103 Read Privilege of Master"]
    #[inline(always)]
    pub fn gpio103read_privilege_of_master(
        &mut self,
    ) -> Gpio103readPrivilegeOfMasterW<Gpio974Spec> {
        Gpio103readPrivilegeOfMasterW::new(self, 24)
    }
}
#[doc = "GPIO Read Privilege Control Register \\#25\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio974::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio974::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gpio974Spec;
impl crate::RegisterSpec for Gpio974Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpio974::R`](R) reader structure"]
impl crate::Readable for Gpio974Spec {}
#[doc = "`write(|w| ..)` method takes [`gpio974::W`](W) writer structure"]
impl crate::Writable for Gpio974Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GPIO974 to value 0xffff_ffff"]
impl crate::Resettable for Gpio974Spec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
